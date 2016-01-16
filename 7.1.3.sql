USE [LogRhythm_LogMart]
GO
/****** Object:  StoredProcedure [dbo].[LogRhythm_LogMart_IdentityInference_HostIdentifierMap_PartitionMaintenance]    Script Date: 01/13/2016 14:53:22 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
/******************************************************************************
*
*  Maintain the partitions in the IdentityInference_HostIdentifierMap table.
*
*  Written: 11/22/13 JR
*
*  Modified:
*
******************************************************************************/
ALTER procedure [dbo].[LogRhythm_LogMart_IdentityInference_HostIdentifierMap_PartitionMaintenance]
as

set nocount on;

declare @CurrentPartition       datetime,
        @RunCount               int,
        @LastPartition          datetime,
        @CurrentPartitionCount  int,
        @SQL                    nvarchar(1000),
        @RetryCounter           tinyint,
        @LoopCounter            tinyint,
        @RequiredPartitionCount int;

SET DEADLOCK_PRIORITY HIGH;

--This procedure runs in a loop so that failures can be retryed
set @RetryCounter = 10;
set @LoopCounter = 0;
while (@LoopCounter <= @RetryCounter)
begin
    set @LoopCounter = @LoopCounter + 1;

    BEGIN TRY
    --------------------------------------------------------------
    --Add partition(s) to main table for new day if required
    --------------------------------------------------------------
    --set @LastPartition to the last partition in table
    select top 1
           @LastPartition = convert(datetime,v.[value])
    from sys.partition_range_values v inner join
         sys.partition_functions f on f.function_id = v.function_id
    where f.name = 'PFLogRhythmIdentityInference'
    order by v.boundary_id DESC;

    --set the run count to the number of days between today and the last partition plus two to account for utc offset
    --and we always want one empty partition on the front
    set @RunCount = DATEDIFF(WEEK,@LastPartition,GETUTCDATE()) + 2;

    while (@RunCount > 0)
    begin
        --set current partition to the earliest (last) partition in main table
        select top 1
               @CurrentPartition = convert(datetime,v.[value])
        from sys.partition_range_values v inner join
             sys.partition_functions f on f.function_id = v.function_id
        where f.name = 'PFLogRhythmIdentityInference'
	    order by v.boundary_id DESC;

        --add one week to current partition
        set @CurrentPartition = DATEADD(WEEK,1,@CurrentPartition);

        --split at the current partition in main table
        print N'Add partition ' + convert(varchar(32),@CurrentPartition);
        alter partition scheme PSLogRhythmIdentityInference next USED [PRIMARY];
        alter partition function PFLogRhythmIdentityInference() SPLIT range (@CurrentPartition);

        set @RunCount = @RunCount - 1;
    end;

    --------------------------------------------------------------
    --Delete partitions that are outside the long ttl
    --------------------------------------------------------------
    select @CurrentPartitionCount = max(p.partition_number)
    from sys.objects o inner join
         sys.partitions p on o.object_id = p.object_id 
    where o.type = 'U'
      and o.name = 'IdentityInference_HostIdentifierMap'
      and p.index_id = 1;

    select @RequiredPartitionCount = convert(int,Value) / 7
    from LogRhythm_LogMart.dbo.SCMaint
    where [Key] = 'TTL_LogMart';

    --make sure 'TTL_LogMart' is valid in SCMaint table
    if (@RequiredPartitionCount is NULL)
    begin
        raiserror('Error: Could not find value for TTL_LogMart in LogRhythm.dbo.SCMaint table.',16,1);
        return 1;    
    end;
    
    set @RequiredPartitionCount = @RequiredPartitionCount + 3  --add additional future days
    set @RunCount = @CurrentPartitionCount - @RequiredPartitionCount

    while (@RunCount > 0)
    begin
        --set current partition to the oldest (first) partition in temp table
        select top 1
               @CurrentPartition = convert(datetime,v.[value])
        from sys.partition_range_values v inner join
             sys.partition_functions f on f.function_id = v.function_id
        where f.name = 'PFLogRhythmIdentityInference'
        order by v.boundary_id ASC;

        BEGIN TRAN

        if (OBJECT_ID('dbo.IdentityInference_HostIdentifierMap_Swap_Temp') is not null)
        begin
            print 'drop table dbo.IdentityInference_HostIdentifierMap_Swap_Temp';
            drop table dbo.IdentityInference_HostIdentifierMap_Swap_Temp;
        end;

        --create a temp table to swap out old IdentityInference_HostIdentifierMap partition
        select top (0) *
        into dbo.IdentityInference_HostIdentifierMap_Swap_Temp
        from dbo.IdentityInference_HostIdentifierMap

        --CREATE CLUSTERED INDEX IX_HostToIdentifierID_LastObserved
        --    ON dbo.IdentityInference_HostIdentifierMap_Swap_Temp(HostToIdentifierID ASC,LastObserved ASC)
        --WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, SORT_IN_TEMPDB = OFF, IGNORE_DUP_KEY = OFF,
        --      DROP_EXISTING = OFF,ONLINE = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON);

        alter table dbo.IdentityInference_HostIdentifierMap_Swap_Temp
        add constraint PK_IdentityInference_HostIdentifierMap_Swap_Temp
        PRIMARY KEY CLUSTERED (HostToIdentifierID ASC, TimeSlot ASC)
        WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF,
              ALLOW_ROW_LOCKS  = ON, ALLOW_PAGE_LOCKS  = ON) ON [PRIMARY];

        --build a check constraint on StatDate
        set @SQL = 'alter table dbo.IdentityInference_HostIdentifierMap_Swap_Temp
                    add CONSTRAINT CK2_IdentityInference_HostIdentifierMap_Swap_Temp_LastObserved
                    CHECK (TimeSlot <= ''' + convert(varchar(24),@CurrentPartition,121) + ''')';

        --print @SQL;
        exec (@SQL);

        --switch the partition from OLD to Swap_Temp
        print N'Partition ' + convert(varchar(32),@CurrentPartition) + ' switched out of IdentityInference_HostIdentifierMap table';
        alter table IdentityInference_HostIdentifierMap SWITCH partition (1) to IdentityInference_HostIdentifierMap_Swap_Temp;

        drop table IdentityInference_HostIdentifierMap_Swap_Temp;

        --merge at current partition on main
        alter partition function PFLogRhythmIdentityInference() merge range (@CurrentPartition);

        set @RunCount = @RunCount - 1;

        COMMIT TRAN
    end;

    END TRY
    BEGIN CATCH
        DECLARE @Error_Severity INT,
                @Error_State    INT,
                @Error_Number   INT,
                @Error_Line     INT,
                @Error_Message  VARCHAR(245);

        SELECT @Error_Severity = Error_Severity(),
               @Error_State = Error_State(),
               @Error_Number = Error_Number(),
               @Error_Line = Error_Line(),
               @Error_Message = Error_Message();

        raiserror('ERROR: %s',@Error_Severity,@Error_State,@Error_Message) with nowait;

        if (@@TRANCOUNT > 1)
            rollback tran;

        --waitfor delay '00:02'  --2 minutes
        continue;

    END CATCH;

    break;  --if no errors then break from job
end;

if (@LoopCounter > @RetryCounter)
begin
    raiserror('ERROR: Too Many Errors - Maintenance Job Failed',16,1);
    return 1;
end;

return 0;
