use allocative::Allocative;
use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::{
    datasets::AnyDataset,
    structs::{
        date_map_vec_to_any_date_map_vec, date_map_vec_to_mut_any_date_map_vec, Amount, AnyBiMap,
        AnyDateMap, AnyHeightMap, BiMap, DateMap, Height, HeightMap, MapKey,
    },
    utils::{
        BYTES_IN_MB, ONE_DAY_IN_DAYS, ONE_MONTH_IN_DAYS, ONE_WEEK_IN_DAYS, ONE_YEAR_IN_DAYS,
        TARGET_BLOCKS_PER_DAY,
    },
};

use super::{
    ComputeData, DateRecapDataset, InsertData, MinInitialStates, RecapDataset, RecapOptions,
};

#[derive(Allocative)]
pub struct MiningDataset {
    min_initial_states: MinInitialStates,

    // Inserted
    pub blocks_mined: DateMap<usize>,
    pub total_blocks_mined: DateMap<usize>,
    pub coinbase: HeightMap<f64>,
    pub coinbase_1d_sum: DateMap<f64>,
    pub coinbase_in_dollars: HeightMap<f32>,
    pub coinbase_in_dollars_1d_sum: DateMap<f32>,
    pub fees: HeightMap<f64>,
    pub fees_1d_sum: DateMap<f64>,
    pub fees_in_dollars: HeightMap<f32>,
    pub fees_in_dollars_1d_sum: DateMap<f32>,
    // Raw
    // pub average_fee_paid: BiMap<f32>,
    // pub max_fee_paid: BiMap<f32>,
    // pub _90th_percentile_fee_paid: BiMap<f32>,
    // pub _75th_percentile_fee_paid: BiMap<f32>,
    // pub median_fee_paid: BiMap<f32>,
    // pub _25th_percentile_fee_paid: BiMap<f32>,
    // pub _10th_percentile_fee_paid: BiMap<f32>,
    // pub min_fee_paid: BiMap<f32>,
    // sat/vB
    // pub average_fee_price: BiMap<f32>,
    // pub max_fee_price: BiMap<f32>,
    // pub _90th_percentile_fee_price: BiMap<f32>,
    // pub _75th_percentile_fee_price: BiMap<f32>,
    // pub median_fee_price: BiMap<f32>,
    // pub _25th_percentile_fee_price: BiMap<f32>,
    // pub _10th_percentile_fee_price: BiMap<f32>,
    // pub min_fee_price: BiMap<f32>,
    // -
    pub subsidy: HeightMap<f64>,
    pub subsidy_1d_sum: DateMap<f64>,
    pub subsidy_in_dollars: HeightMap<f32>,
    pub subsidy_in_dollars_1d_sum: DateMap<f32>,
    pub last_coinbase: DateMap<f64>,
    pub last_coinbase_in_dollars: DateMap<f32>,
    pub last_fees: DateMap<f64>,
    pub last_fees_in_dollars: DateMap<f32>,
    pub last_subsidy: DateMap<f64>,
    pub last_subsidy_in_dollars: DateMap<f32>,
    pub difficulty: BiMap<f64>,
    pub block_size: HeightMap<f32>,   // in MB
    pub block_weight: HeightMap<f32>, // in MB
    pub block_vbytes: HeightMap<u64>,
    pub block_interval: HeightMap<u32>, // in s

    // Computed
    pub annualized_issuance: DateMap<f64>, // Same as subsidy_1y_sum
    pub blocks_mined_1d_target: DateMap<usize>,
    pub blocks_mined_1m_sma: DateMap<f32>,
    pub blocks_mined_1m_sum: DateMap<usize>,
    pub blocks_mined_1m_target: DateMap<usize>,
    pub blocks_mined_1w_sma: DateMap<f32>,
    pub blocks_mined_1w_sum: DateMap<usize>,
    pub blocks_mined_1w_target: DateMap<usize>,
    pub blocks_mined_1y_sum: DateMap<usize>,
    pub blocks_mined_1y_target: DateMap<usize>,
    pub cumulative_block_size: BiMap<f32>,
    pub subsidy_1y_sum: DateMap<f64>,
    pub subsidy_in_dollars_1y_sum: DateMap<f64>,
    pub cumulative_subsidy: BiMap<f64>,
    pub cumulative_subsidy_in_dollars: BiMap<f32>,
    pub coinbase_1y_sum: DateMap<f64>,
    pub coinbase_in_dollars_1y_sum: DateMap<f64>,
    pub coinbase_in_dollars_1d_sum_1y_sma: DateMap<f32>,
    pub cumulative_coinbase: BiMap<f64>,
    pub cumulative_coinbase_in_dollars: BiMap<f32>,
    pub fees_1y_sum: DateMap<f64>,
    pub fees_in_dollars_1y_sum: DateMap<f64>,
    pub cumulative_fees: BiMap<f64>,
    pub cumulative_fees_in_dollars: BiMap<f32>,
    pub inflation_rate: DateMap<f64>,
    pub yearly_inflation_rate: DateMap<f64>,
    pub subsidy_to_coinbase_ratio: HeightMap<f64>,
    pub subsidy_to_coinbase_1d_ratio: DateMap<f64>,
    pub fees_to_coinbase_ratio: HeightMap<f64>,
    pub fees_to_coinbase_1d_ratio: DateMap<f64>,
    pub hash_rate: DateMap<f64>,
    pub hash_rate_1w_sma: DateMap<f32>,
    pub hash_rate_1m_sma: DateMap<f32>,
    pub hash_rate_2m_sma: DateMap<f32>,
    pub hash_price: DateMap<f64>,
    pub difficulty_adjustment: DateMap<f64>,
    pub block_size_recap: DateRecapDataset<f32>, // in MB
    pub block_weight_recap: DateRecapDataset<f32>, // in MB
    pub block_vbytes_recap: DateRecapDataset<u64>,
    pub block_interval_recap: DateRecapDataset<u32>, // in s
    pub puell_multiple: DateMap<f32>,
    // pub hash_price_in_dollars: DateMap<f64>,
    // pub hash_price_30d_volatility: BiMap<f32>,
    // difficulty_adjustment
    // next_difficulty_adjustment
    // op return fees
    // inscriptions fees
    // until adjustement
    // until halving in days
    // until halving in blocks
}

impl MiningDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            total_blocks_mined: DateMap::new_bin(1, &f("total_blocks_mined")),
            blocks_mined: DateMap::new_bin(1, &f("blocks_mined")),
            coinbase: HeightMap::new_bin(1, &f("coinbase")),
            coinbase_1d_sum: DateMap::new_bin(1, &f("coinbase_1d_sum")),
            coinbase_in_dollars: HeightMap::new_bin(1, &f("coinbase_in_dollars")),
            coinbase_in_dollars_1d_sum: DateMap::new_bin(1, &f("coinbase_in_dollars_1d_sum")),
            coinbase_1y_sum: DateMap::new_bin(1, &f("coinbase_1y_sum")),
            coinbase_in_dollars_1y_sum: DateMap::new_bin(1, &f("coinbase_in_dollars_1y_sum")),
            coinbase_in_dollars_1d_sum_1y_sma: DateMap::new_bin(
                1,
                &f("coinbase_in_dollars_1d_sum_1y_sma"),
            ),
            cumulative_coinbase: BiMap::new_bin(1, &f("cumulative_coinbase")),
            cumulative_coinbase_in_dollars: BiMap::new_bin(1, &f("cumulative_coinbase_in_dollars")),
            fees: HeightMap::new_bin(1, &f("fees")),
            fees_1d_sum: DateMap::new_bin(1, &f("fees_1d_sum")),
            fees_in_dollars: HeightMap::new_bin(1, &f("fees_in_dollars")),
            fees_in_dollars_1d_sum: DateMap::new_bin(1, &f("fees_in_dollars_1d_sum")),
            fees_1y_sum: DateMap::new_bin(1, &f("fees_1y_sum")),
            fees_in_dollars_1y_sum: DateMap::new_bin(1, &f("fees_in_dollars_1y_sum")),
            cumulative_fees: BiMap::new_bin(1, &f("cumulative_fees")),
            cumulative_fees_in_dollars: BiMap::new_bin(1, &f("cumulative_fees_in_dollars")),
            subsidy: HeightMap::new_bin(1, &f("subsidy")),
            subsidy_1d_sum: DateMap::new_bin(1, &f("subsidy_1d_sum")),
            subsidy_in_dollars: HeightMap::new_bin(1, &f("subsidy_in_dollars")),
            subsidy_in_dollars_1d_sum: DateMap::new_bin(1, &f("subsidy_in_dollars_1d_sum")),
            subsidy_1y_sum: DateMap::new_bin(1, &f("subsidy_1y_sum")),
            subsidy_in_dollars_1y_sum: DateMap::new_bin(1, &f("subsidy_in_dollars_1y_sum")),
            cumulative_subsidy: BiMap::new_bin(1, &f("cumulative_subsidy")),
            cumulative_subsidy_in_dollars: BiMap::new_bin(1, &f("cumulative_subsidy_in_dollars")),

            subsidy_to_coinbase_ratio: HeightMap::new_bin(1, &f("subsidy_to_coinbase_ratio")),
            subsidy_to_coinbase_1d_ratio: DateMap::new_bin(1, &f("subsidy_to_coinbase_1d_ratio")),
            fees_to_coinbase_ratio: HeightMap::new_bin(1, &f("fees_to_coinbase_ratio")),
            fees_to_coinbase_1d_ratio: DateMap::new_bin(1, &f("fees_to_coinbase_1d_ratio")),

            annualized_issuance: DateMap::new_bin(1, &f("annualized_issuance")),
            inflation_rate: DateMap::new_bin(2, &f("inflation_rate")),
            yearly_inflation_rate: DateMap::new_bin(1, &f("yearly_inflation_rate")),

            last_subsidy: DateMap::new_bin(1, &f("last_subsidy")),
            last_subsidy_in_dollars: DateMap::new_bin(1, &f("last_subsidy_in_dollars")),
            last_coinbase: DateMap::new_bin(1, &f("last_coinbase")),
            last_coinbase_in_dollars: DateMap::new_bin(1, &f("last_coinbase_in_dollars")),
            last_fees: DateMap::new_bin(1, &f("last_fees")),
            last_fees_in_dollars: DateMap::new_bin(1, &f("last_fees_in_dollars")),

            blocks_mined_1d_target: DateMap::new_bin(1, &f("blocks_mined_1d_target")),
            blocks_mined_1w_sma: DateMap::new_bin(1, &f("blocks_mined_1w_sma")),
            blocks_mined_1m_sma: DateMap::new_bin(1, &f("blocks_mined_1m_sma")),

            blocks_mined_1w_sum: DateMap::new_bin(1, &f("blocks_mined_1w_sum")),
            blocks_mined_1m_sum: DateMap::new_bin(1, &f("blocks_mined_1m_sum")),
            blocks_mined_1y_sum: DateMap::new_bin(1, &f("blocks_mined_1y_sum")),

            blocks_mined_1w_target: DateMap::new_bin(1, &f("blocks_mined_1w_target")),
            blocks_mined_1m_target: DateMap::new_bin(1, &f("blocks_mined_1m_target")),
            blocks_mined_1y_target: DateMap::new_bin(1, &f("blocks_mined_1y_target")),

            difficulty: BiMap::new_bin(1, &f("difficulty")),
            difficulty_adjustment: DateMap::new_bin(1, &f("difficulty_adjustment")),
            block_size: HeightMap::new_bin(1, &f("block_size")),
            block_size_recap: RecapDataset::import(
                &f("block_size_1d"),
                RecapOptions::default()
                    .add_sum()
                    .add_average()
                    .add_max()
                    .add_90p()
                    .add_75p()
                    .add_median()
                    .add_25p()
                    .add_10p()
                    .add_min(),
            )?,
            cumulative_block_size: BiMap::new_bin(1, &f("cumulative_block_size")),
            block_weight: HeightMap::new_bin(1, &f("block_weight")),
            block_weight_recap: RecapDataset::import(
                &f("block_weight_1d"),
                RecapOptions::default()
                    .add_average()
                    .add_max()
                    .add_90p()
                    .add_75p()
                    .add_median()
                    .add_25p()
                    .add_10p()
                    .add_min(),
            )?,
            block_vbytes: HeightMap::new_bin(1, &f("block_vbytes")),
            block_vbytes_recap: RecapDataset::import(
                &f("block_vbytes_1d"),
                RecapOptions::default()
                    .add_average()
                    .add_max()
                    .add_90p()
                    .add_75p()
                    .add_median()
                    .add_25p()
                    .add_10p()
                    .add_min(),
            )?,
            // block_vbytes_1d_sma: HeightMap::new_bin(1, &f("block_vbytes")),
            block_interval: HeightMap::new_bin(2, &f("block_interval")),
            block_interval_recap: RecapDataset::import(
                &f("block_interval_1d"),
                RecapOptions::default()
                    .add_average()
                    .add_max()
                    .add_90p()
                    .add_75p()
                    .add_median()
                    .add_25p()
                    .add_10p()
                    .add_min(),
            )?,
            hash_rate: DateMap::new_bin(1, &f("hash_rate")),
            hash_rate_1w_sma: DateMap::new_bin(1, &f("hash_rate_1w_sma")),
            hash_rate_1m_sma: DateMap::new_bin(1, &f("hash_rate_1m_sma")),
            hash_rate_2m_sma: DateMap::new_bin(1, &f("hash_rate_2m_sma")),
            hash_price: DateMap::new_bin(1, &f("hash_price")),
            puell_multiple: DateMap::new_bin(1, &f("puell_multiple")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert(
        &mut self,
        &InsertData {
            date_first_height,
            height,
            coinbase,
            fees,
            date_blocks_range,
            is_date_last_block,
            block_price,
            date,
            difficulty,
            block_size,
            block_vbytes,
            block_weight,
            block_interval,
            ..
        }: &InsertData,
    ) {
        self.coinbase.insert(height, coinbase.to_btc());

        let coinbase_in_dollars = self
            .coinbase_in_dollars
            .insert(height, (block_price * coinbase).to_dollar() as f32);

        let sumed_fees = Amount::from_sat(fees.iter().map(|amount| amount.to_sat()).sum());

        self.fees.insert(height, sumed_fees.to_btc());

        let sumed_fees_in_dollars = self
            .fees_in_dollars
            .insert(height, (block_price * sumed_fees).to_dollar() as f32);

        let subsidy = coinbase - sumed_fees;
        self.subsidy.insert(height, subsidy.to_btc());

        let subsidy_in_dollars = self
            .subsidy_in_dollars
            .insert(height, (block_price * subsidy).to_dollar() as f32);

        self.difficulty.height.insert(height, difficulty);

        self.block_size
            .insert(height, block_size as f32 / BYTES_IN_MB as f32);
        self.block_weight
            .insert(height, block_weight as f32 / BYTES_IN_MB as f32);
        self.block_vbytes.insert(height, block_vbytes);
        self.block_interval.insert(height, *block_interval);

        if is_date_last_block {
            self.coinbase_1d_sum
                .insert(date, self.coinbase.sum_range(date_blocks_range));

            self.coinbase_in_dollars_1d_sum
                .insert(date, self.coinbase_in_dollars.sum_range(date_blocks_range));

            self.fees_1d_sum
                .insert(date, self.fees.sum_range(date_blocks_range));

            self.fees_in_dollars_1d_sum
                .insert(date, self.fees_in_dollars.sum_range(date_blocks_range));

            self.subsidy_1d_sum
                .insert(date, self.subsidy.sum_range(date_blocks_range));

            self.subsidy_in_dollars_1d_sum
                .insert(date, self.subsidy_in_dollars.sum_range(date_blocks_range));

            self.last_coinbase.insert(date, coinbase.to_btc());

            self.last_coinbase_in_dollars
                .insert(date, coinbase_in_dollars);

            self.last_subsidy.insert(date, subsidy.to_btc());

            self.last_subsidy_in_dollars
                .insert(date, subsidy_in_dollars);

            self.last_fees.insert(date, sumed_fees.to_btc());

            self.last_fees_in_dollars
                .insert(date, sumed_fees_in_dollars);

            let total_blocks_mined = self.total_blocks_mined.insert(date, height.to_usize() + 1);

            self.blocks_mined
                .insert(date, total_blocks_mined - date_first_height.to_usize());

            self.difficulty.date.insert(date, difficulty);
        }
    }

    pub fn compute(
        &mut self,
        &ComputeData { heights, dates, .. }: &ComputeData,
        first_height: &mut DateMap<Height>,
        last_height: &mut DateMap<Height>,
    ) {
        self.blocks_mined_1w_sum.multi_insert_last_x_sum(
            dates,
            &mut self.blocks_mined,
            ONE_WEEK_IN_DAYS,
        );

        self.blocks_mined_1m_sum.multi_insert_last_x_sum(
            dates,
            &mut self.blocks_mined,
            ONE_MONTH_IN_DAYS,
        );

        self.blocks_mined_1y_sum.multi_insert_last_x_sum(
            dates,
            &mut self.blocks_mined,
            ONE_YEAR_IN_DAYS,
        );

        self.subsidy_1y_sum.multi_insert_last_x_sum(
            dates,
            &mut self.subsidy_1d_sum,
            ONE_YEAR_IN_DAYS,
        );

        self.subsidy_in_dollars_1y_sum.multi_insert_last_x_sum(
            dates,
            &mut self.subsidy_in_dollars_1d_sum,
            ONE_YEAR_IN_DAYS,
        );

        self.cumulative_subsidy
            .height
            .multi_insert_cumulative(heights, &mut self.subsidy);
        self.cumulative_subsidy
            .date
            .multi_insert_cumulative(dates, &mut self.subsidy_1d_sum);

        self.cumulative_subsidy_in_dollars
            .height
            .multi_insert_cumulative(heights, &mut self.subsidy_in_dollars);
        self.cumulative_subsidy_in_dollars
            .date
            .multi_insert_cumulative(dates, &mut self.subsidy_in_dollars_1d_sum);

        self.fees_1y_sum
            .multi_insert_last_x_sum(dates, &mut self.fees_1d_sum, ONE_YEAR_IN_DAYS);

        self.fees_in_dollars_1y_sum.multi_insert_last_x_sum(
            dates,
            &mut self.fees_in_dollars_1d_sum,
            ONE_YEAR_IN_DAYS,
        );

        self.cumulative_fees
            .height
            .multi_insert_cumulative(heights, &mut self.fees);
        self.cumulative_fees
            .date
            .multi_insert_cumulative(dates, &mut self.fees_1d_sum);

        self.cumulative_fees_in_dollars
            .height
            .multi_insert_cumulative(heights, &mut self.fees_in_dollars);
        self.cumulative_fees_in_dollars
            .date
            .multi_insert_cumulative(dates, &mut self.fees_in_dollars_1d_sum);

        self.coinbase_1y_sum.multi_insert_last_x_sum(
            dates,
            &mut self.coinbase_1d_sum,
            ONE_YEAR_IN_DAYS,
        );

        self.coinbase_in_dollars_1y_sum.multi_insert_last_x_sum(
            dates,
            &mut self.coinbase_in_dollars_1d_sum,
            ONE_YEAR_IN_DAYS,
        );

        self.coinbase_in_dollars_1d_sum_1y_sma
            .multi_insert_simple_average(
                dates,
                &mut self.coinbase_in_dollars_1d_sum,
                ONE_YEAR_IN_DAYS,
            );

        self.cumulative_coinbase
            .height
            .multi_insert_cumulative(heights, &mut self.coinbase);
        self.cumulative_coinbase
            .date
            .multi_insert_cumulative(dates, &mut self.coinbase_1d_sum);

        self.cumulative_coinbase_in_dollars
            .height
            .multi_insert_cumulative(heights, &mut self.coinbase_in_dollars);
        self.cumulative_coinbase_in_dollars
            .date
            .multi_insert_cumulative(dates, &mut self.coinbase_in_dollars_1d_sum);

        self.subsidy_to_coinbase_ratio.multi_insert_percentage(
            heights,
            &mut self.subsidy,
            &mut self.coinbase,
        );
        self.subsidy_to_coinbase_1d_ratio.multi_insert_percentage(
            dates,
            &mut self.subsidy_1d_sum,
            &mut self.coinbase_1d_sum,
        );

        self.fees_to_coinbase_ratio.multi_insert_percentage(
            heights,
            &mut self.fees,
            &mut self.coinbase,
        );
        self.fees_to_coinbase_1d_ratio.multi_insert_percentage(
            dates,
            &mut self.fees_1d_sum,
            &mut self.coinbase_1d_sum,
        );

        self.annualized_issuance.multi_insert_last_x_sum(
            dates,
            &mut self.subsidy_1d_sum,
            ONE_YEAR_IN_DAYS,
        );

        self.inflation_rate.multi_insert_simple_transform(
            dates,
            &mut self.subsidy_1d_sum,
            |subsidy_1d_sum, date| {
                subsidy_1d_sum * ONE_YEAR_IN_DAYS as f64
                    / self.cumulative_subsidy.date.get_or_import(date).unwrap()
                    * 100.0
            },
        );

        self.yearly_inflation_rate.multi_insert_percentage(
            dates,
            &mut self.annualized_issuance,
            &mut self.cumulative_subsidy.date,
        );

        self.blocks_mined_1d_target
            .multi_insert_const(dates, TARGET_BLOCKS_PER_DAY);

        self.blocks_mined_1w_target
            .multi_insert_const(dates, ONE_WEEK_IN_DAYS * TARGET_BLOCKS_PER_DAY);

        self.blocks_mined_1m_target
            .multi_insert_const(dates, ONE_MONTH_IN_DAYS * TARGET_BLOCKS_PER_DAY);

        self.blocks_mined_1y_target
            .multi_insert_const(dates, ONE_YEAR_IN_DAYS * TARGET_BLOCKS_PER_DAY);

        self.blocks_mined_1w_sma.multi_insert_simple_average(
            dates,
            &mut self.blocks_mined,
            ONE_WEEK_IN_DAYS,
        );

        self.blocks_mined_1m_sma.multi_insert_simple_average(
            dates,
            &mut self.blocks_mined,
            ONE_MONTH_IN_DAYS,
        );

        self.cumulative_block_size
            .height
            .multi_insert_cumulative(heights, &mut self.block_size);

        self.cumulative_block_size.date.multi_insert_last(
            dates,
            &mut self.cumulative_block_size.height,
            last_height,
        );

        // https://hashrateindex.com/blog/what-is-bitcoins-hashrate/
        self.hash_rate.multi_insert(dates, |date| {
            let blocks_mined = self.blocks_mined.get_or_import(date).unwrap();

            let difficulty = self.difficulty.date.get_or_import(date).unwrap();

            ((blocks_mined as f64 / date.get_day_completion() * TARGET_BLOCKS_PER_DAY as f64)
                * difficulty
                * 2.0_f64.powi(32))
                / 600.0
                / 1_000_000_000_000_000_000.0
        });

        self.hash_rate_1w_sma.multi_insert_simple_average(
            dates,
            &mut self.hash_rate,
            ONE_WEEK_IN_DAYS,
        );

        self.hash_rate_1m_sma.multi_insert_simple_average(
            dates,
            &mut self.hash_rate,
            ONE_MONTH_IN_DAYS,
        );

        self.hash_rate_2m_sma.multi_insert_simple_average(
            dates,
            &mut self.hash_rate,
            2 * ONE_MONTH_IN_DAYS,
        );

        self.hash_price.multi_insert(dates, |date| {
            let coinbase_in_dollars = self.coinbase_in_dollars_1d_sum.get_or_import(date).unwrap();

            let hashrate = self.hash_rate.get_or_import(date).unwrap();

            coinbase_in_dollars as f64 / hashrate / 1_000.0
        });

        self.puell_multiple.multi_insert_divide(
            dates,
            &mut self.coinbase_in_dollars_1d_sum,
            &mut self.coinbase_in_dollars_1d_sum_1y_sma,
        );
        self.puell_multiple.multi_insert_divide(
            dates,
            &mut self.coinbase_in_dollars_1d_sum,
            &mut self.coinbase_in_dollars_1d_sum_1y_sma,
        );

        self.difficulty_adjustment.multi_insert_percentage_change(
            dates,
            &mut self.difficulty.date,
            ONE_DAY_IN_DAYS,
        );

        dates.iter().for_each(|date| {
            let first = first_height.get_or_import(date).unwrap();
            let last = last_height.get_or_import(date).unwrap();

            self.block_size_recap.compute(
                *date,
                &mut self
                    .block_size
                    .get_or_import_range_inclusive(first, last)
                    .into_iter()
                    .map(OrderedFloat)
                    .collect_vec(),
            );

            self.block_weight_recap.compute(
                *date,
                &mut self
                    .block_weight
                    .get_or_import_range_inclusive(first, last)
                    .into_iter()
                    .map(OrderedFloat)
                    .collect_vec(),
            );

            self.block_vbytes_recap.compute(
                *date,
                &mut self.block_vbytes.get_or_import_range_inclusive(first, last),
            );

            self.block_interval_recap.compute(
                *date,
                &mut self
                    .block_interval
                    .get_or_import_range_inclusive(first, last),
            );
        })
    }
}

impl AnyDataset for MiningDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }

    fn to_inserted_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.difficulty]
    }

    fn to_inserted_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.difficulty]
    }

    fn to_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![
            &self.coinbase_1d_sum,
            &self.coinbase_in_dollars_1d_sum,
            &self.fees_1d_sum,
            &self.fees_in_dollars_1d_sum,
            &self.subsidy_1d_sum,
            &self.subsidy_in_dollars_1d_sum,
            &self.total_blocks_mined,
            &self.blocks_mined,
            &self.last_subsidy,
            &self.last_subsidy_in_dollars,
            &self.last_coinbase,
            &self.last_coinbase_in_dollars,
            &self.last_fees,
            &self.last_fees_in_dollars,
        ]
    }

    fn to_inserted_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        vec![
            &mut self.coinbase_1d_sum,
            &mut self.coinbase_in_dollars_1d_sum,
            &mut self.fees_1d_sum,
            &mut self.fees_in_dollars_1d_sum,
            &mut self.subsidy_1d_sum,
            &mut self.subsidy_in_dollars_1d_sum,
            &mut self.total_blocks_mined,
            &mut self.blocks_mined,
            &mut self.last_subsidy,
            &mut self.last_subsidy_in_dollars,
            &mut self.last_coinbase,
            &mut self.last_coinbase_in_dollars,
            &mut self.last_fees,
            &mut self.last_fees_in_dollars,
        ]
    }

    fn to_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![
            &self.coinbase,
            &self.coinbase_in_dollars,
            &self.fees,
            &self.fees_in_dollars,
            &self.subsidy,
            &self.subsidy_in_dollars,
            &self.block_size,
            &self.block_weight,
            &self.block_vbytes,
            &self.block_interval,
        ]
    }

    fn to_inserted_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        vec![
            &mut self.coinbase,
            &mut self.coinbase_in_dollars,
            &mut self.fees,
            &mut self.fees_in_dollars,
            &mut self.subsidy,
            &mut self.subsidy_in_dollars,
            &mut self.block_size,
            &mut self.block_weight,
            &mut self.block_vbytes,
            &mut self.block_interval,
        ]
    }

    fn to_computed_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.cumulative_coinbase,
            &self.cumulative_coinbase_in_dollars,
            &self.cumulative_fees,
            &self.cumulative_fees_in_dollars,
            &self.cumulative_subsidy,
            &self.cumulative_subsidy_in_dollars,
            &self.cumulative_block_size,
        ]
    }

    fn to_computed_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.cumulative_coinbase,
            &mut self.cumulative_coinbase_in_dollars,
            &mut self.cumulative_fees,
            &mut self.cumulative_fees_in_dollars,
            &mut self.cumulative_subsidy,
            &mut self.cumulative_subsidy_in_dollars,
            &mut self.cumulative_block_size,
        ]
    }

    fn to_computed_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![
            &self.subsidy_to_coinbase_ratio,
            &self.fees_to_coinbase_ratio,
        ]
    }

    fn to_computed_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        vec![
            &mut self.subsidy_to_coinbase_ratio,
            &mut self.fees_to_coinbase_ratio,
        ]
    }

    fn to_computed_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        [
            &self.blocks_mined_1d_target as &(dyn AnyDateMap + Send + Sync),
            &self.blocks_mined_1w_sma,
            &self.blocks_mined_1m_sma,
            &self.blocks_mined_1w_sum,
            &self.blocks_mined_1m_sum,
            &self.blocks_mined_1y_sum,
            &self.blocks_mined_1w_target,
            &self.blocks_mined_1m_target,
            &self.blocks_mined_1y_target,
            &self.subsidy_1y_sum,
            &self.subsidy_in_dollars_1y_sum,
            &self.coinbase_1y_sum,
            &self.coinbase_in_dollars_1y_sum,
            &self.coinbase_in_dollars_1d_sum_1y_sma,
            &self.fees_to_coinbase_1d_ratio,
            &self.annualized_issuance,
            &self.fees_1y_sum,
            &self.fees_in_dollars_1y_sum,
            &self.inflation_rate,
            &self.yearly_inflation_rate,
            &self.subsidy_to_coinbase_1d_ratio,
            &self.hash_rate,
            &self.hash_rate_1w_sma,
            &self.hash_rate_1m_sma,
            &self.hash_rate_2m_sma,
            &self.hash_price,
            &self.puell_multiple,
            &self.difficulty_adjustment,
        ]
        .into_iter()
        .chain(date_map_vec_to_any_date_map_vec(
            self.block_size_recap.as_vec(),
        ))
        .chain(date_map_vec_to_any_date_map_vec(
            self.block_vbytes_recap.as_vec(),
        ))
        .chain(date_map_vec_to_any_date_map_vec(
            self.block_weight_recap.as_vec(),
        ))
        .chain(date_map_vec_to_any_date_map_vec(
            self.block_interval_recap.as_vec(),
        ))
        .collect_vec()
    }

    fn to_computed_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        [
            &mut self.blocks_mined_1d_target as &mut dyn AnyDateMap,
            &mut self.blocks_mined_1w_sma,
            &mut self.blocks_mined_1m_sma,
            &mut self.blocks_mined_1w_sum,
            &mut self.blocks_mined_1m_sum,
            &mut self.blocks_mined_1y_sum,
            &mut self.blocks_mined_1w_target,
            &mut self.blocks_mined_1m_target,
            &mut self.blocks_mined_1y_target,
            &mut self.annualized_issuance,
            &mut self.subsidy_1y_sum,
            &mut self.subsidy_in_dollars_1y_sum,
            &mut self.fees_to_coinbase_1d_ratio,
            &mut self.inflation_rate,
            &mut self.yearly_inflation_rate,
            &mut self.subsidy_to_coinbase_1d_ratio,
            &mut self.coinbase_1y_sum,
            &mut self.coinbase_in_dollars_1y_sum,
            &mut self.coinbase_in_dollars_1d_sum_1y_sma,
            &mut self.fees_1y_sum,
            &mut self.fees_in_dollars_1y_sum,
            &mut self.hash_rate,
            &mut self.hash_rate_1w_sma,
            &mut self.hash_rate_1m_sma,
            &mut self.hash_rate_2m_sma,
            &mut self.hash_price,
            &mut self.puell_multiple,
            &mut self.difficulty_adjustment,
        ]
        .into_iter()
        .chain(date_map_vec_to_mut_any_date_map_vec(
            self.block_size_recap.as_mut_vec(),
        ))
        .chain(date_map_vec_to_mut_any_date_map_vec(
            self.block_vbytes_recap.as_mut_vec(),
        ))
        .chain(date_map_vec_to_mut_any_date_map_vec(
            self.block_weight_recap.as_mut_vec(),
        ))
        .chain(date_map_vec_to_mut_any_date_map_vec(
            self.block_interval_recap.as_mut_vec(),
        ))
        .collect_vec()
    }
}
