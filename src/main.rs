// We'll be generating these algorithmically to find
// the best possible outcome
#[derive(Copy, Clone)]
struct CharStats {
	hp: f32,
	atk: f32,
	em: f32,
	elemental_bonus: f32,
	crit_rate: f32,
	crit_damage: f32
}

// Her base stats
const TAO_BASE: CharStats = CharStats {
	hp: 15552.0,
	atk: 106.43,
	crit_rate: 5.0,
	crit_damage: 88.4,
	elemental_bonus: 33.0,
	em: 0.0,
};

impl ToString for CharStats {
	fn to_string(&self) -> String {
		format!(
			"Stats {{\n\tHP: {},\n\tATK: {},\n\tEM: {},\n\tELEM%: {},\n\tCR: {},\n\tCD: {}\n}}",
			self.hp,
			self.atk,
			self.em,
			self.elemental_bonus,
			self.crit_rate,
			self.crit_damage
		)
	}
}
// The damage formula expressed as a multivariate function
fn damage(
	base_dmg: f32,
	base_dmg_multiplier: f32,
	additive_dmg_bonus: f32,
	dmg_bonus: f32,
	crit_rate: f32,
	crit_damage: f32,
	amplifying_reaction: f32
) -> f32 {
	// Effective crit multiplier evaluated as n - number of hits, approaches infinity
	let crit = 1.0 + (crit_rate / 100.0).clamp(0.0, 1.0) * crit_damage / 100.0;

	// Assume we're fighting Masanori lvl. 70 if this switch is on
	let masanori = true;

	let (enemy_def_multiplier,enemy_res_multiplier) = match masanori {
		true => (0.52, 0.9),
		false => (1.0, 1.0)
	};

	(base_dmg * base_dmg_multiplier + additive_dmg_bonus) *
	(1.0 + dmg_bonus) * crit * enemy_def_multiplier *
	enemy_res_multiplier * amplifying_reaction
}

// Below section contains various buffs that can be applied
// to Hu Tao. They will be used later
fn stats_tao_skill(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.atk += stats.hp * 0.0626;
	stats
}

fn stats_soss_base(
	mut stats: CharStats,
) -> CharStats {
	stats.crit_rate += 44.1;
	stats.atk += 542.0;
	stats
}

fn stats_soss_buff(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.atk += 0.52 * stats.em;
	stats.atk += 0.28 * 3.0 * stats.em;
	stats
}

fn stats_homa_base(
	mut stats: CharStats
) -> CharStats {
	stats.crit_damage += 66.4;
	stats.atk += 608.0;
	stats
}

fn stats_homa_buff(
	base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.hp += base.hp * 0.2;
	stats.atk += base.hp * 0.018;
	stats
}

fn stats_bennett_buff(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.atk += 1000.0;
	stats.elemental_bonus += 15.0;
	stats
}

fn stats_mh_buff(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.crit_rate += 36.0;
	stats
}

// Assume we always roll into % and never flat. Ignore minrolls.
fn stats_raw(
	base: CharStats,
	weapon: fn(CharStats) -> CharStats,
	dynamic_buffs: Vec<fn(CharStats, CharStats) -> CharStats>,
	mainstat_em: f32,
	mainstat_hp: f32,
	mainstat_atk: f32,
	mainstat_elemental: f32,
	mainstat_cr: f32,
	mainstat_cd: f32,
	hp_rolls: usize,
	atk_rolls: usize,
	em_rolls: usize,
	crit_rate_rolls: usize,
	crit_damage_rolls: usize,
) -> CharStats {
	let base = weapon(base);
	let mut dynamic = CharStats {
		hp: 4780.0 + base.hp * (1.0 + (hp_rolls as f32 * 5.83) / 100.0 + mainstat_hp / 100.0),
		atk: 311.0 + base.atk * (1.0 + (atk_rolls as f32 * 5.83) / 100.0 + mainstat_atk / 100.0),
		crit_rate: base.crit_rate + mainstat_cr + crit_rate_rolls as f32 * 3.89,
		crit_damage: base.crit_damage + mainstat_cd + crit_damage_rolls as f32 * 7.77,
		elemental_bonus: base.elemental_bonus + mainstat_elemental,
		em: base.em + mainstat_em + em_rolls as f32 * 23.31,
	};
	for buff in dynamic_buffs {
		dynamic = buff(base, dynamic);
	}
	dynamic
}

// A wrapper over stats_raw that decomposes parameters from vectors
fn stats(
	base: CharStats,
	weapon: fn(CharStats) -> CharStats,
	dynamic_buffs: Vec<fn(CharStats, CharStats) -> CharStats>,
	mainstats: [f32; 6],
	rolls: &[usize; 5],
) -> CharStats {
	stats_raw(
		base,
		weapon,
		dynamic_buffs,
		mainstats[0],
		mainstats[1],
		mainstats[2],
		mainstats[3],
		mainstats[4],
		mainstats[5],
		rolls[0],
		rolls[1],
		rolls[2],
		rolls[3],
		rolls[4]
	)
}

// The damage calculation for her CA at talent lvl. 10
fn tao_ca_vape(
	tao_stats: CharStats,
	external_dmg_bonus: f32,
	flat_ca_buff: f32,
	reaction_bonus: f32
) -> f32 {
	let ca_multiplier = 242.57 / 100.0;
	let vape_multiplier = 1.5 * (1.0 + (2.78 * tao_stats.em) / (1400.0 + tao_stats.em) + reaction_bonus);

	damage(
		tao_stats.atk * ca_multiplier,
		1.0,
		flat_ca_buff,
		(tao_stats.elemental_bonus + external_dmg_bonus) / 100.0,
		tao_stats.crit_rate,
		tao_stats.crit_damage,
		vape_multiplier
	)
}

fn main() {
	// We want to investigate various mainstat variations
	let arti_mainstat_distributions = [
		// EM      HP    ATK   BONUS%  CR    CD
		[  561.0,  0.0,  0.0,  0.0,    0.0,  0.0   ],  // Triple EM
		[  374.0,  0.0,  0.0,  0.0,    0.0,  62.2  ],  // Double EM + CD
		[  374.0,  0.0,  0.0,  0.0,    31.1, 0.0   ],  // Double EM + CR
		[  187.0,  0.0,  0.0,  46.6,   0.0,  62.2  ],  // EM + Pyro + CD
		[  187.0,  46.6, 0.0,  0.0,    31.1, 0.0   ],  // EM + HP + CR
		[  187.0,  46.6, 0.0,  0.0,    0.0,  62.2  ],  // EM + HP + CD
		[  0.0,    46.6, 0.0,  46.6,   0.0,  62.2  ],  // HP + Pyro + CD
		[  0.0,    46.6, 0.0,  46.6,   31.1, 0.0   ],  // HP + Pyro + CR
	];

	// Assuming we have 20 rolls to distribute across substats,
	// find all possible substat combinations (ignore minrolls)
	let mut arti_substat_distributions = Vec::new();
	for hp in 0..21 {
		for atk in 0..(21-hp) {
			for em in 0..(21-atk-hp) {
				for cr in 0..(21-atk-hp-em) {
					for cd in 0..(21-atk-hp-em-cr) {
						arti_substat_distributions.push([hp, atk, em, cr, cd]);
					}
				}
			}
		}
	}

	// Scan through all of the possible mainstat and substat
	// combinations and look for the highest value
	let mut max_damage = 0.0;
	let mut max_distribution = None;
	let mut max_stats = None;

	for mainstats in arti_mainstat_distributions {
		for substats in &arti_substat_distributions {
			let stats = stats(
				TAO_BASE,
				stats_soss_base,		// This is the weapon base stat function
				vec![					// This is a list of all the dynamic buffs
					stats_soss_buff,
					stats_tao_skill,
					stats_bennett_buff
				],
				mainstats,
				substats
			);

			let damage = tao_ca_vape(
				stats,
				50.0 + 75.0, // Ext DMG% bonus - Shimenawa + Furina
				0.0,		 // Ext flat bonus
				0.0			 // Ext reaction bonus (eg. crimson)
			);

			if damage > max_damage {
				max_damage = damage;

				let mut distribution = mainstats.to_vec();
				distribution.extend(substats.map(|x| x as f32));

				max_distribution = Some(distribution);
				max_stats = Some(stats);
			}
		}
	}

	// This will print the optimal build
	// It will output the number of substat rolls denoted with 'r' at end.
	if let Some(max_distribution) = max_distribution {
		println!("OPTIMAL ARTIFACT STATS:");
		let stats = vec!["EM", "HP", "ATK", "BONUS%", "CR", "CD", "HP%r", "ATK%r", "EMr", "CRr", "CDr"];
		for i in 0..11 {
			println!("{}\t{}", stats[i], max_distribution[i]);
		}
	}

	// Print a summary of the stats (incl. buffs)
	println!("\nSTAT SUMMARY:");
	println!("{}", max_stats.unwrap().to_string());
	println!("CA:\t{} damage per vape", max_damage);
}