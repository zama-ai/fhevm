#[cfg(test)]
mod tests {
    use crate::scheduler::tests::utils::TestContext;
    use crate::scheduler::traits::Commands;
    use crate::scheduler::traits::Events;
    use crate::scheduler::BlockContext;
    use crate::scheduler::Handle;
    use crate::scheduler::{computation_scheduler::ComputationScheduler, messages};
    use fhevm_engine_common::types::SupportedFheOperations;
    use std::time::SystemTime;

    pub fn blob(id: u8) -> Handle {
        let mut h = [0u8; 32];
        h[0] = id;
        h
    }

    #[test]
    fn test_simple_chain() {
        let mut schd = ComputationScheduler::default();

        let log1 = messages::FheLog {
            output_handle: blob(1),
            dependencies: vec![],
            fhe_operation: SupportedFheOperations::FheTrivialEncrypt,
            is_scalar: false,
            is_allowed: false,
            created_at: SystemTime::now(),
            block_info: BlockContext {
                txn_hash: [0u8; 32],
                block_number: 1,
                block_hash: [0u8; 32],
            },
        };

        let log2 = messages::FheLog {
            output_handle: blob(2),
            dependencies: vec![blob(1)],
            fhe_operation: SupportedFheOperations::FheAdd,
            is_scalar: false,
            is_allowed: true,
            created_at: SystemTime::now(),
            block_info: log1.block_info.clone(),
        };

        schd.on_fhe_log_msg(&log1, false);
        schd.on_fhe_log_msg(&log2, false);
        schd.retrieve_executable_partitions();
    }

    #[test]
    fn test_generate_partitions() {
        let folder = "./test-outputs";
        let mut schd = ComputationScheduler::default();
        let ctx_tx1 = TestContext::new(1, [1u8; 32], [10u8; 32]);

        let log_event_1 = &ctx_tx1.event_log(
            blob(1),
            vec![],
            SupportedFheOperations::FheTrivialEncrypt,
            false,
        );
        let _ = schd.on_fhe_log_msg(log_event_1, false);

        let log_event_2 = &ctx_tx1.event_log(
            blob(2),
            vec![blob(1)],
            SupportedFheOperations::FheAdd,
            false,
        );

        let _ = schd.on_fhe_log_msg(log_event_2, false);

        let l3 = schd.on_fhe_log_msg(
            &ctx_tx1.event_log(blob(3), vec![blob(2)], SupportedFheOperations::FheAdd, true),
            false,
        );

        // TX2 context
        let ctx_tx2 = TestContext::new(1, [2u8; 32], [10u8; 32]);
        let _ = schd.on_fhe_log_msg(
            &ctx_tx2.event_log(
                blob(10),
                vec![blob(3)],
                SupportedFheOperations::FheAdd,
                true,
            ),
            true,
        );

        let _ = schd.on_fhe_log_msg(
            &ctx_tx2.event_log(
                blob(11),
                vec![blob(3)],
                SupportedFheOperations::FheAdd,
                true,
            ),
            true,
        );

        if std::path::Path::new(folder).exists() {
            schd.export_graphs(folder);
        }

        let _ = schd.on_fhe_log_msg(
            &ctx_tx2.event_log(
                blob(12),
                vec![blob(11)],
                SupportedFheOperations::FheAdd,
                true,
            ),
            true,
        );

        let _ = schd.on_fhe_log_msg(
            &ctx_tx2.event_log(
                blob(13),
                vec![blob(11)],
                SupportedFheOperations::FheAdd,
                true,
            ),
            true,
        );

        let partitions = schd.retrieve_executable_partitions();
        if std::path::Path::new(folder).exists() {
            schd.export_graphs(folder);
        }

        for cycle in 1..=3 {
            let partitions = schd.retrieve_executable_partitions();
            for (i, partition) in partitions.iter().enumerate() {
                println!("Cycle:{} Partition {}: {}", cycle, i, partition);
                schd.on_partition_completed(&partition);
            }
        }

        let partitions = schd.retrieve_executable_partitions();
        assert!(
            partitions.is_empty(),
            "Expected no executable partitions, but found some"
        );

        /*
        Cycle:1 Partition 0: ExecutablePartition { hash: "0400fdd0", computations: [01000000, 02000000, 03000000, 0a000000] }
        Cycle:2 Partition 0: ExecutablePartition { hash: "0100dd13", computations: [0b000000] }
        Cycle:3 Partition 0: ExecutablePartition { hash: "01003e1e", computations: [0c000000] }
        Cycle:3 Partition 1: ExecutablePartition { hash: "0100702e", computations: [0d000000] }
        */
    }
}
