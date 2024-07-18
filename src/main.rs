mod buffs;
mod characters;
mod rotations;

// We'll be generating these algorithmically to find
// the best possible outcome
#[derive(Copy, Clone)]
pub struct CharStats {
	hp: f32,
	atk: f32,
	em: f32,
	dmg_bonus: f32,
	na_bonus: f32,
	ca_bonus: f32,
	reaction_bonus: f32,
	crit_rate: f32,
	crit_damage: f32
}

impl ToString for CharStats {
	fn to_string(&self) -> String {
		format!(
			"Stats {{\n\tHP: {},\n\tATK: {},\n\tEM: {},\n\tDMG%: {},\n\tCR: {},\n\tCD: {}\n}}",
			self.hp,
			self.atk,
			self.em,
			self.dmg_bonus,
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

	// Assume we're fighting Masanori lvl. 90 if this switch is on
	let masanori = true;

	let (enemy_def_multiplier, enemy_res_multiplier) = match masanori {
		true => (0.5, 0.9),
		false => (1.0, 1.0)
	};

	(base_dmg * base_dmg_multiplier + additive_dmg_bonus) *
	(1.0 + dmg_bonus) * crit * enemy_def_multiplier *
	enemy_res_multiplier * amplifying_reaction
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
		dmg_bonus: base.dmg_bonus + mainstat_elemental,
		reaction_bonus: 0.0,
		na_bonus: 0.0,
		ca_bonus: 0.0,
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

fn forward_vape_multiplier(
	trigger: &CharStats
) -> f32 {
	2.0 * (1.0 + (2.78 * trigger.em) / (1400.0 + trigger.em) + trigger.reaction_bonus)
}

fn shark_na_bite(
	shark: &CharStats,
	momentum: usize,
	vape: bool
) -> f32 {
	let mut wave_bonus = momentum as f32 * 0.13 * shark.hp;
	let na_multiplier = 52.1 / 100.0;
	let vape_multiplier = match vape {
		true => forward_vape_multiplier(&shark),
		false => 1.0
	};

	if momentum == 3 {
		wave_bonus += 0.391 * shark.hp;
	}

	damage(
		shark.hp * na_multiplier,
		1.0,
		wave_bonus,
		(shark.dmg_bonus + shark.na_bonus) / 100.0,
		shark.crit_rate,
		shark.crit_damage,
		vape_multiplier
	)
}

fn shark_na1(
	shark: &CharStats,
	vape: bool
) -> f32 {
	let na_multiplier = 92.5 / 100.0;
	let vape_multiplier = match vape {
		true => forward_vape_multiplier(&shark),
		false => 1.0
	};

	damage(
		shark.atk * na_multiplier,
		1.0,
		0.0,
		(shark.dmg_bonus + shark.na_bonus) / 100.0,
		shark.crit_rate,
		shark.crit_damage,
		vape_multiplier
	)
}

fn shark_na2(
	shark: &CharStats,
	vape: bool
) -> f32 {
	let na_multiplier = 80.3 / 100.0;
	let vape_multiplier = match vape {
		true => forward_vape_multiplier(&shark),
		false => 1.0
	};

	damage(
		shark.atk * na_multiplier,
		1.0,
		0.0,
		(shark.dmg_bonus + shark.na_bonus) / 100.0,
		shark.crit_rate,
		shark.crit_damage,
		vape_multiplier
	)
}

fn shark_na3(
	shark: &CharStats,
	vape: bool
) -> f32 {
	let na_multiplier = 126.1 / 100.0;
	let vape_multiplier = match vape {
		true => forward_vape_multiplier(&shark),
		false => 1.0
	};

	damage(
		shark.atk * na_multiplier,
		1.0,
		0.0,
		(shark.dmg_bonus + shark.na_bonus) / 100.0,
		shark.crit_rate,
		shark.crit_damage,
		vape_multiplier
	)
}

fn surfing_time_base(
	mut base: CharStats,
) -> CharStats {
	base.crit_damage += 88.2;
	base.atk += 542.0;
	base
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

	// Assuming we have 25 rolls to distribute across substats,
	// find all possible substat combinations (ignore minrolls)
	let mut arti_substat_distributions = Vec::new();
	for hp in 0..26 {
		for atk in 0..(26-hp) {
			for em in 0..(26-atk-hp) {
				for cr in 0..(26-atk-hp-em) {
					for cd in 0..(26-atk-hp-em-cr) {
						arti_substat_distributions.push([hp, atk, em, cr, cd]);
					}
				}
			}
		}
	}

	// We're gonna keep track of all builds
	let mut all_damage = Vec::new();

	for mainstats in arti_mainstat_distributions {
		for substats in &arti_substat_distributions {
			let stats = stats(
				characters::SHARK,
				surfing_time_base,		// This is the weapon base stat function
				vec![					// This is a list of all the dynamic buffs
					buffs::shime,
					buffs::furina_burst,
					buffs::hydro_resonance,
					buffs::instructor_share,
				],
				mainstats,
				substats
			);

			let mut damage = 0.0;
			
			// The duration of her skill seems to be around 6s idfk
			// Just assume she bites two times after applying 3 stacks each time
			// The last one she can only apply one stack before the skill ends
			damage += shark_na_bite(&stats, 3, true);
			damage += shark_na_bite(&stats, 3, true);
			damage += shark_na_bite(&stats, 2, true);

			let mut distribution = mainstats.to_vec();
			distribution.extend(substats.map(|x| x as f32));
			all_damage.push((damage, distribution, stats));
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

	// Print a summary of the stats (incl. buffs)
	println!("\nSTAT SUMMARY:");
	println!("{}", stats.to_string());
	println!("DPR: {}", damage);
}