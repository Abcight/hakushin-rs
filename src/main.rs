use buffs::{sac_jade_buff, MagicBoxed};

mod buffs;
mod characters;
mod rotations;
mod calculators;

// We'll be generating these algorithmically to find
// the best possible outcome
#[derive(Copy, Clone)]
pub struct CharStats {
	hp: f32,
	atk: f32,
	em: f32,
	dmg_bonus: f32,
	na_bonus: f32,
	na_bonus_flat: f32,
	skill_bonus: f32,
	ca_bonus: f32,
	reaction_bonus: f32,
	crit_rate: f32,
	crit_damage: f32,
	res_shred: f32
}

impl ToString for CharStats {
	fn to_string(&self) -> String {
		format!(
			"HP: {},\nATK: {},\nEM: {},\nDMG%: {},\nCR: {},\nCD: {}\n--------------",
			self.hp,
			self.atk,
			self.em,
			self.dmg_bonus,
			self.crit_rate,
			self.crit_damage
		)
	}
}

fn stats_kqms(
	base: CharStats,
	weapon: impl Fn(CharStats) -> CharStats,
	dynamic_buffs: Vec<&dyn Fn(CharStats, CharStats) -> CharStats>,
	mainstat_em: f32,
	mainstat_hp: f32,
	mainstat_atk: f32,
	mainstat_elemental: f32,
	mainstat_cr: f32,
	mainstat_cd: f32,
	mut hp_rolls: isize,
	mut atk_rolls: isize,
	mut em_rolls: isize,
	mut crit_rate_rolls: isize,
	mut crit_damage_rolls: isize,
) -> CharStats {
	// max 10 per stat!
	hp_rolls = hp_rolls.min(10);
	atk_rolls = atk_rolls.min(10);
	hp_rolls = hp_rolls.min(10);
	em_rolls = em_rolls.min(10);
	crit_rate_rolls = crit_rate_rolls.min(10);
	crit_damage_rolls = crit_damage_rolls.min(10);

	// we need to decrease the amount of rolls in substats
	// based on the mainstats of artifacts
	hp_rolls -= ((mainstat_hp / 46.6) * 2.0).round() as isize;
	atk_rolls -= ((mainstat_atk / 46.6) * 2.0).round() as isize;
	em_rolls -= ((mainstat_em / 187.0) * 2.0).round() as isize;
	crit_rate_rolls -= ((mainstat_cr / 31.1) * 2.0).round() as isize;
	crit_damage_rolls -= ((mainstat_cd / 62.2) * 2.0).round() as isize;

	let base = weapon(base);
	let mut dynamic = CharStats {
		hp: 4780.0 + base.hp + base.hp * (hp_rolls as f32 * 4.96 + mainstat_hp) / 100.0,
		atk: 311.0 + base.atk + base.atk * ((atk_rolls as f32 * 4.96) / 100.0 + mainstat_atk / 100.0),
		crit_rate: base.crit_rate + mainstat_cr + crit_rate_rolls as f32 * 3.31,
		crit_damage: base.crit_damage + mainstat_cd + crit_damage_rolls as f32 * 6.62,
		dmg_bonus: base.dmg_bonus + mainstat_elemental,
		reaction_bonus: 0.0,
		na_bonus: 0.0,
		skill_bonus: 0.0,
		ca_bonus: 0.0,
		em: base.em + mainstat_em + em_rolls as f32 * 19.82,
		res_shred: 0.0,
		na_bonus_flat: 0.0,
	};
	dynamic.hp += base.hp * (2.0 * 4.96) / 100.0;
	dynamic.hp += 2.0 * 253.0;
	dynamic.atk += base.atk * (2.0 * 4.96) / 100.0;
	dynamic.atk += 2.0 * 16.54;
	dynamic.em += 2.0 * 16.82;
	dynamic.crit_rate += 2.0 * 3.31;
	dynamic.crit_damage += 2.0 * 6.62;
	for buff in dynamic_buffs {
		dynamic = buff(base, dynamic);
	}
	dynamic
}

// Assume we always roll into % and never flat. Ignore minrolls.
fn stats_raw(
	base: CharStats,
	weapon: impl Fn(CharStats) -> CharStats,
	dynamic_buffs: Vec<&dyn Fn(CharStats, CharStats) -> CharStats>,
	mainstat_em: f32,
	mainstat_hp: f32,
	mainstat_atk: f32,
	mainstat_elemental: f32,
	mainstat_cr: f32,
	mainstat_cd: f32,
	hp_rolls: isize,
	atk_rolls: isize,
	em_rolls: isize,
	crit_rate_rolls: isize,
	crit_damage_rolls: isize,
) -> CharStats {
	let base = weapon(base);
	let mut dynamic = CharStats {
		hp: 4780.0 + base.hp + base.hp * (hp_rolls as f32 * 5.83 + mainstat_hp) / 100.0,
		atk: 311.0 + base.atk * (1.0 + (atk_rolls as f32 * 5.83) / 100.0 + mainstat_atk / 100.0),
		crit_rate: base.crit_rate + mainstat_cr + crit_rate_rolls as f32 * 3.89,
		crit_damage: base.crit_damage + mainstat_cd + crit_damage_rolls as f32 * 7.77,
		dmg_bonus: base.dmg_bonus + mainstat_elemental,
		reaction_bonus: 0.0,
		na_bonus: 0.0,
		skill_bonus: 0.0,
		ca_bonus: 0.0,
		em: base.em + mainstat_em + em_rolls as f32 * 23.31,
		res_shred: 0.0,
		na_bonus_flat: 0.0,
	};
	for buff in dynamic_buffs {
		dynamic = buff(base, dynamic);
	}
	dynamic
}

// A wrapper over stats_raw that decomposes parameters from vectors
fn stats(
	base: CharStats,
	weapon: impl Fn(CharStats) -> CharStats,
	dynamic_buffs: Vec<&dyn Fn(CharStats, CharStats) -> CharStats>,
	mainstats: &[f32; 6],
	rolls: &[usize; 5],
) -> CharStats {
	stats_kqms(
		base,
		weapon,
		dynamic_buffs,
		mainstats[0],
		mainstats[1],
		mainstats[2],
		mainstats[3],
		mainstats[4],
		mainstats[5],
		rolls[0] as isize,
		rolls[1] as isize,
		rolls[2] as isize,
		rolls[3] as isize,
		rolls[4] as isize
	)
}

fn main() {
	// We want to investigate various mainstat variations
	let arti_mainstat_distributions = vec![
		// EM      HP    ATK   BONUS%  CR    CD
		[  561.0,  0.0,  0.0,  0.0,    0.0,  0.0   ],  // Triple EM
		[  374.0,  0.0,  0.0,  0.0,    0.0,  62.2  ],  // Double EM + CD
		[  374.0,  0.0,  0.0,  0.0,    31.1, 0.0   ],  // Double EM + CR
		[  187.0,  0.0,  0.0,  46.6,   0.0,  62.2  ],  // EM + Dmg + CD
		[  187.0,  46.6, 0.0,  0.0,    31.1, 0.0   ],  // EM + HP + CR
		[  187.0,  46.6, 0.0,  0.0,    0.0,  62.2  ],  // EM + HP + CD
		[  0.0,    46.6, 0.0,  46.6,   0.0,  62.2  ],  // HP + Dmg + CD
		[  0.0,    46.6, 0.0,  46.6,   31.1, 0.0   ],  // HP + Dmg + CR
		[  187.0,  0.0,  46.6, 0.0,    31.1, 0.0   ],  // EM + ATK + CR
		[  187.0,  0.0,  46.6, 0.0,    0.0,  62.2  ],  // EM + ATK + CD
		[  0.0,    0.0,  46.6, 46.6,   0.0,  62.2  ],  // ATK + Dmg + CD
		[  0.0,    0.0,  46.6, 46.6,   31.1, 0.0   ],  // ATK + Dmg + CR
	];

	// Assuming we have n max-rolls to distribute across substats,
	// find all possible substat combinations (ignore minrolls)
	let num_maxrolls = 20;
	let mut arti_substat_distributions = Vec::new();
	for hp in 0..(1+num_maxrolls) {
		for atk in 0..(1+num_maxrolls-hp) {
			for em in 0..(1+num_maxrolls-atk-hp) {
				for cr in 0..(1+num_maxrolls-atk-hp-em) {
					for cd in 0..(1+num_maxrolls-atk-hp-em-cr) {
						arti_substat_distributions.push([hp, atk, em, cr, cd]);
					}
				}
			}
		}
	}

	let catalysts: Vec<(&str, &dyn Fn(CharStats) -> CharStats, Box<dyn Fn(CharStats, CharStats) -> CharStats>)> = vec![
		("Surfing Time R1", &buffs::surfing_time_base, buffs::surfing_time_buff(1, 4).boxed()),
		("Surfing Time R5", &buffs::surfing_time_base, buffs::surfing_time_buff(5, 4).boxed()),
		("Sacrificial Jade R1", &buffs::sac_jade_base, buffs::sac_jade_buff(1).boxed()),
		("Sacrificial Jade R5", &buffs::sac_jade_base, buffs::sac_jade_buff(5).boxed()),
		("Ring of Yaxche R1", &buffs::ceiba_base, buffs::ceiba_buff(1).boxed()),
		("Ring of Yaxche R5", &buffs::ceiba_base, buffs::ceiba_buff(5).boxed()),
		("Magic Guide R5", &buffs::magic_guide_base, buffs::magic_guide_buff(false).boxed()),
		("Solar Pearl R1", &buffs::solar_pearl_base, buffs::solar_pearl_buff(1).boxed()),
		("Solar Pearl R5", &buffs::solar_pearl_base, buffs::solar_pearl_buff(5).boxed()),
		("The Widsith R1 [ATK]", &buffs::widsith_base, buffs::widsith_buff(1, 0).boxed()),
		("The Widsith R1 [DMG]", &buffs::widsith_base, buffs::widsith_buff(1, 1).boxed()),
		("The Widsith R1 [EM]", &buffs::widsith_base, buffs::widsith_buff(1, 2).boxed()),
		("The Widsith R1 [None]", &buffs::widsith_base, buffs::widsith_buff(1, 3).boxed()),
		("The Widsith R5 [ATK]", &buffs::widsith_base, buffs::widsith_buff(5, 0).boxed()),
		("The Widsith R5 [DMG]", &buffs::widsith_base, buffs::widsith_buff(5, 1).boxed()),
		("The Widsith R5 [EM]", &buffs::widsith_base, buffs::widsith_buff(5, 2).boxed()),
		("The Widsith R5 [None]", &buffs::widsith_base, buffs::widsith_buff(5, 3).boxed()),
		("Floating Dreams R1", &buffs::floating_dreams_base, buffs::floating_dreams_buff(1, 0, 3).boxed()),
		("Floating Dreams R5", &buffs::floating_dreams_base, buffs::floating_dreams_buff(5, 0, 3).boxed()),
		("Tome of Eternal Flow R1", &buffs::tome_base, buffs::tome_buff(1, 0).boxed()),
		("Tome of Eternal Flow R5", &buffs::tome_base, buffs::tome_buff(5, 0).boxed()),
	];

	let spears: Vec<(&str, &dyn Fn(CharStats) -> CharStats, Box<dyn Fn(CharStats, CharStats) -> CharStats>)> = vec![
		("Lumidouce Elegy R1", &buffs::lumidouce_base, buffs::lumidouce_buff(1, 2).boxed()),
		("Lumidouce Elegy R5", &buffs::lumidouce_base, buffs::lumidouce_buff(5, 2).boxed()),
		("Staff of Homa R1", &buffs::homa_base, buffs::homa_buff(false).boxed()),
		("Scarlet Sands R1", &buffs::soss_base, buffs::soss_buff(1).boxed()),
		("Dragon's Bane R1", &buffs::dragons_base, buffs::dragons_buff(1).boxed()),
		("Dragon's Bane R5", &buffs::dragons_base, buffs::dragons_buff(5).boxed()),
		("Missive Windspear R1", &buffs::missive_base, buffs::missive_buff(1).boxed()),
		("Missive Windspear R5", &buffs::missive_base, buffs::missive_buff(5).boxed()),
	];

	// calculators::weapon_calculator(
	// 	catalysts,
	// 	arti_mainstat_distributions,
	// 	arti_substat_distributions,
	// 	"Ring of Yaxche R5",
	// 	|mainstats, substats, base, buff| {
	// 		rotations::shark_furina_thoma_nahida(mainstats, substats, base, buff)
	// 	}
	// );

	// We're gonna keep track of all builds
	let mut all_damage = Vec::with_capacity(
		arti_mainstat_distributions.len() *
		arti_substat_distributions.len()
	);

	for mainstats in arti_mainstat_distributions {
		for substats in &arti_substat_distributions {
			let stats_base = stats(
				characters::SHARK,
				&buffs::sac_jade_base,
				vec![
					&buffs::sac_jade_buff(5)
				],
				&mainstats,
				substats
			);

			let damage = rotations::shark_furina_thoma_nahida(
				&mainstats,
				&substats,
				buffs::sac_jade_base,
				buffs::sac_jade_buff(5)
			);

			let mut distribution = mainstats.to_vec();
			distribution.extend(substats.map(|x| x as f32));
			all_damage.push((damage, distribution, stats_base));
		}
	}

	// Decide target distribution
	all_damage.sort_by_key(|x| x.0 as usize);
	let median = all_damage.get(all_damage.len() / 2).unwrap();
	let minimum = all_damage.first().unwrap();
	let maximum = all_damage.last().unwrap();

	// Decompose from target
	let (damage, distribution, stats) = maximum;

	// This will print the optimal build
	// It will output the number of substat rolls denoted with 'r' at end.
	let labels = vec!["EM", "HP", "ATK", "BONUS%", "CR", "CD", "HP%r", "ATK%r", "EMr", "CRr", "CDr"];
	println!("OPTIMAL ARTIFACT MAINSTATS:");
	for i in 0..6 {
		println!("{}\t{}", labels[i], distribution[i]);
	}
	println!("\nOPTIMAL ARTIFACT SUBSTAT ROLLS:");
	for i in 6..11 {
		println!("{}\t{}", labels[i], distribution[i]);
	}

	println!("\nBASE STATS WITH ARTI:");
	println!("{}", stats.to_string());

	// Print a summary of the stats (incl. buffs)
	println!("DPR: {}", damage);
}