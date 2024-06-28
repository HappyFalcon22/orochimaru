//! Circuit for checking the constraints of the original memory trace record
extern crate alloc;
use crate::constraints::{
    common::CircuitExtension,
    gadgets::{
        ConvertedTraceRecord, GreaterThanConfig, LookUpTables, Queries, Table,
        TraceRecordWitnessTable,
    },
};
use alloc::{format, vec, vec::Vec};
use core::marker::PhantomData;
use ff::{Field, PrimeField};
use halo2_proofs::{
    circuit::{Layouter, Region, SimpleFloorPlanner, Value},
    plonk::{Circuit, Column, ConstraintSystem, Error, Expression, Fixed, Selector},
    poly::Rotation,
};
use rand::thread_rng;
#[derive(Clone, Copy, Debug)]
/// Config for trace record that is sorted by time_log
pub(crate) struct OriginalMemoryConfig<F: Field + PrimeField> {
    /// The original trace circuit
    pub(crate) trace_record: TraceRecordWitnessTable<F>,
    /// The selectors
    pub(crate) selector: Column<Fixed>,
    pub(crate) selector_zero: Selector,
    /// The config for checking the current time log is bigger than the previous one
    pub(crate) greater_than: GreaterThanConfig<F, 3>,
    /// The lookup table
    pub(crate) lookup_tables: LookUpTables,
}
// Current constraints in this configure are:
// 1) time[0]=0
// 2) time[i]<time[i+1]
// There will be more constraints in the config when we support PUSH and POP
impl<F: Field + PrimeField> OriginalMemoryConfig<F> {
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        trace_record: TraceRecordWitnessTable<F>,
        lookup_tables: LookUpTables,
        alpha_power: Vec<Expression<F>>,
    ) -> Self {
        let selector = meta.fixed_column();
        let selector_zero = meta.selector();
        // This is used to check that time_log[i]<time_log[i+1] for all i
        // we set address_included=false because we do not need address here
        let greater_than = GreaterThanConfig::<F, 3>::configure(
            meta,
            trace_record,
            alpha_power,
            lookup_tables,
            selector,
            false,
        );
        // Check that time_log[0]=0
        meta.create_gate("first accessed memory is at time 0", |meta| {
            let selector_zero = meta.query_selector(selector_zero);
            let time_log = Queries::new(meta, trace_record, Rotation::cur()).time_log;
            let mut time = time_log[0].clone();
            for t in time_log.iter().skip(1) {
                time = time * Expression::Constant(F::from(256_u64)) + t.clone();
            }
            vec![selector_zero * time]
        });
        OriginalMemoryConfig {
            trace_record,
            selector,
            selector_zero,
            greater_than,
            lookup_tables,
        }
    }
}

/// Circuit for original trace record
#[derive(Default)]
pub(crate) struct OriginalMemoryCircuit<F: Field + PrimeField> {
    /// The original memory trace record
    pub(crate) original_trace_record: Vec<ConvertedTraceRecord<F>>,
    pub(crate) _marker: PhantomData<F>,
}

/// Implement the CircuitExtension trait for the OriginalMemoryCircuit
impl<F: Field + PrimeField> CircuitExtension<F> for OriginalMemoryCircuit<F> {
    fn synthesize_with_layouter(
        &self,
        config: Self::Config,
        layouter: &mut impl Layouter<F>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "original memory trace region",
            |mut region| {
                for i in 0..self.original_trace_record.len() {
                    self.original_memory_assign(&mut region, config, i)?;
                }
                config.lookup_tables.size40_table.load(&mut region)?;
                config.lookup_tables.size256_table.load(&mut region)?;
                config.lookup_tables.size2_table.load(&mut region)?;
                Ok(())
            },
        )?;
        Ok(())
    }
}

impl<F: Field + PrimeField> Circuit<F> for OriginalMemoryCircuit<F> {
    type Config = OriginalMemoryConfig<F>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    // Configure the circuit
    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let rng = thread_rng();

        // The elements of the trace record
        let trace_record = TraceRecordWitnessTable::<F>::new(meta);

        // Lookup tables
        let lookup_tables = LookUpTables {
            size256_table: Table::<256>::construct(meta),
            size40_table: Table::<40>::construct(meta),
            size2_table: Table::<2>::construct(meta),
        };
        // The random challenges
        // For debugging purpose, we let alpha to be uniformly distributed
        // Later, one can force the prover to commit the memory traces first, then
        // let alpha to be the hash of the commitment
        let alpha = Expression::Constant(F::random(rng));
        let mut temp = Expression::Constant(F::ONE);
        let mut alpha_power: Vec<Expression<F>> = vec![temp.clone()];
        for _ in 0..8 {
            temp = temp * alpha.clone();
            alpha_power.push(temp.clone());
        }

        OriginalMemoryConfig::configure(meta, trace_record, lookup_tables, alpha_power)
    }

    // Assign the witness values to the entire witness table and their constraints
    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        self.synthesize_with_layouter(config, &mut layouter)
    }
}

impl<F: Field + PrimeField> OriginalMemoryCircuit<F> {
    // Assign the witness values to the offset-th row of the witness table
    fn original_memory_assign(
        &self,
        region: &mut Region<'_, F>,
        config: OriginalMemoryConfig<F>,
        offset: usize,
    ) -> Result<(), Error> {
        // Handle the case offset=0
        if offset == 0 {
            let (cur_address, cur_time_log, cur_instruction, cur_value) =
                self.original_trace_record[offset].get_tuple();

            // Turn on the first selector when offset=0
            config.selector_zero.enable(region, offset)?;

            // Assign the address witness
            for (i, j) in cur_address.iter().zip(config.trace_record.address) {
                region.assign_advice(
                    || format!("address{}", offset),
                    j,
                    offset,
                    || Value::known(*i),
                )?;
            }
            // Assign the time_log witness
            for (i, j) in cur_time_log.iter().zip(config.trace_record.time_log) {
                region.assign_advice(
                    || format!("time_log{}", offset),
                    j,
                    offset,
                    || Value::known(*i),
                )?;
            }
            // Assign the instruction witness
            region.assign_advice(
                || format!("instruction{}", offset),
                config.trace_record.instruction,
                offset,
                || Value::known(cur_instruction),
            )?;
            // Assign the value witness
            for (i, j) in cur_value.iter().zip(config.trace_record.value) {
                region.assign_advice(
                    || format!("value{}", offset),
                    j,
                    offset,
                    || Value::known(*i),
                )?;
            }
        }
        // Handle the case offset >= 1
        else {
            // Get the current and the previous trace record
            let (cur_address, cur_time_log, cur_instruction, cur_value) =
                self.original_trace_record[offset].get_tuple();
            let (_prev_address, prev_time_log, _prev_instruction, _prev_value) =
                self.original_trace_record[offset - 1].get_tuple();
            let limb_vector: Vec<u8> = (0..8).collect();
            // Find the minimal index such that cur is not equal to prev
            let find_result = limb_vector
                .iter()
                .zip(&cur_time_log)
                .zip(&prev_time_log)
                .find(|((_, a), b)| a != b);
            let zero = F::ZERO;
            let ((index, cur_limb), prev_limb) = if cfg!(test) {
                find_result.unwrap_or(((&8, &zero), &zero))
            } else {
                find_result.expect("two trace records cannot have equal time log")
            };
            let difference = *cur_limb - *prev_limb;

            // Assign the selector to be one at the current row
            region.assign_fixed(
                || "selector",
                config.selector,
                offset,
                || Value::known(F::ONE),
            )?;

            // Assign the address witness
            for (i, j) in cur_address.iter().zip(config.trace_record.address) {
                region.assign_advice(
                    || format!("address{}", offset),
                    j,
                    offset,
                    || Value::known(*i),
                )?;
            }

            // Assign the time_log witness
            for (i, j) in cur_time_log.iter().zip(config.trace_record.time_log) {
                region.assign_advice(
                    || format!("time_log{}", offset),
                    j,
                    offset,
                    || Value::known(*i),
                )?;
            }

            // Assign the instruction witness
            region.assign_advice(
                || format!("instruction{}", offset),
                config.trace_record.instruction,
                offset,
                || Value::known(cur_instruction),
            )?;

            // Assign the value witness
            for (i, j) in cur_value.iter().zip(config.trace_record.value) {
                region.assign_advice(
                    || format!("value{}", offset),
                    j,
                    offset,
                    || Value::known(*i),
                )?;
            }

            // Assign the difference of time witness
            region.assign_advice(
                || format!("difference of time_log{}", offset),
                config.greater_than.difference,
                offset,
                || Value::known(difference),
            )?;

            // Assign the inverse of the time difference witness
            region.assign_advice(
                || format!("time_log difference_inverse{}", offset),
                config.greater_than.difference_inverse,
                offset,
                || Value::known(difference.invert().expect("cannot find inverse")),
            )?;

            // Assign the first_difference_limb witness
            config
                .greater_than
                .first_difference_limb
                .assign(region, offset, *index)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::constraints::original_memory_circuit::{
        ConvertedTraceRecord, OriginalMemoryCircuit,
    };
    use halo2_proofs::dev::MockProver;
    use halo2curves::bn256::Fr as Fp;
    extern crate alloc;
    extern crate std;
    use alloc::{vec, vec::Vec};
    use std::marker::PhantomData;
    // Common function to build and test the circuit
    fn build_and_test_circuit(trace: Vec<ConvertedTraceRecord<Fp>>, k: u32) {
        let circuit = OriginalMemoryCircuit::<Fp> {
            original_trace_record: trace,
            _marker: PhantomData,
        };

        let prover = MockProver::run(k, &circuit, vec![]).expect("Cannot run the circuit");
        assert_eq!(prover.verify(), Ok(()));
    }
    #[test]
    fn test_one_trace() {
        let trace0 = ConvertedTraceRecord {
            address: [Fp::from(0); 32],
            time_log: [Fp::from(0); 8],
            instruction: Fp::from(1),
            value: [Fp::from(63); 32],
        };
        let trace = vec![trace0];
        build_and_test_circuit(trace, 10);
    }

    #[test]
    #[should_panic]
    fn test_wrong_starting_time() {
        // Trace with time_log starts at 1
        let trace0 = ConvertedTraceRecord {
            address: [Fp::from(0); 32],
            time_log: [Fp::from(1); 8],
            instruction: Fp::from(1),
            value: [Fp::from(63); 32],
        };
        build_and_test_circuit(vec![trace0], 10);
    }

    #[test]
    fn test_multiple_traces() {
        let trace0 = ConvertedTraceRecord {
            address: [Fp::from(0); 32],
            time_log: [Fp::from(0); 8],
            instruction: Fp::from(1),
            value: [Fp::from(63); 32],
        };
        let trace1 = ConvertedTraceRecord {
            address: [Fp::from(1); 32],
            time_log: [Fp::from(1); 8],
            instruction: Fp::from(1),
            value: [Fp::from(63); 32],
        };
        let trace2 = ConvertedTraceRecord {
            address: [Fp::from(2); 32],
            time_log: [Fp::from(2); 8],
            instruction: Fp::from(1),
            value: [Fp::from(63); 32],
        };
        let trace3 = ConvertedTraceRecord {
            address: [Fp::from(3); 32],
            time_log: [Fp::from(3); 8],
            instruction: Fp::from(1),
            value: [Fp::from(63); 32],
        };
        build_and_test_circuit(vec![trace0, trace1, trace2, trace3], 10);
    }

    #[test]
    #[should_panic]
    fn test_identical_trace() {
        let trace0 = ConvertedTraceRecord {
            address: [Fp::from(0); 32],
            time_log: [Fp::from(0); 8],
            instruction: Fp::from(1),
            value: [Fp::from(63); 32],
        };
        let trace1 = ConvertedTraceRecord {
            address: [Fp::from(0); 32],
            time_log: [Fp::from(1); 8],
            instruction: Fp::from(1),
            value: [Fp::from(63); 32],
        };
        let trace2 = ConvertedTraceRecord {
            address: [Fp::from(0); 32],
            time_log: [Fp::from(1); 8],
            instruction: Fp::from(1),
            value: [Fp::from(63); 32],
        };
        build_and_test_circuit(vec![trace0, trace1, trace2], 10);
    }

    #[test]
    #[should_panic]
    fn test_invalid_time_order() {
        let trace0 = ConvertedTraceRecord {
            address: [Fp::from(0); 32],
            time_log: [Fp::from(1); 8],
            instruction: Fp::from(1),
            value: [Fp::from(63); 32],
        };
        let trace1 = ConvertedTraceRecord {
            address: [Fp::from(1); 32],
            time_log: [Fp::from(0); 8],
            instruction: Fp::from(1),
            value: [Fp::from(63); 32],
        };

        build_and_test_circuit(vec![trace0, trace1], 10);
    }
}
