use crate::*;

/////////////////////////////
///////// Common ////////////
/////////////////////////////

// The damage formula expressed as a multivariate function
fn damage(
	base_dmg: f32,
	base_dmg_multiplier: f32,
	additive_dmg_bonus: f32,
	dmg_bonus: f32,
	crit_rate: f32,
	crit_damage: f32,
	res_shred: f32,
	amplifying_reaction: f32
) -> f32 {
	// Effective crit multiplier evaluated as n - number of hits, approaches infinity
	let crit = 1.0 + (crit_rate / 100.0).clamp(0.0, 1.0) * crit_damage / 100.0;

	// Assume we're fighting Masanori lvl. 90 if this switch is on
	let masanori = true;

	let (enemy_def_multiplier, mut enemy_res_multiplier) = match masanori {
		true => (0.48, 0.9),
		false => (1.0, 1.0)
	};

	let over_shred = enemy_res_multiplier + res_shred - 1.0;
	if over_shred > 0.0 {
		enemy_res_multiplier = 1.0 + over_shred * 0.5;
	} else {
		enemy_res_multiplier += res_shred;
	}

	(base_dmg * base_dmg_multiplier + additive_dmg_bonus) *
	(1.0 + dmg_bonus) * crit * enemy_def_multiplier *
	enemy_res_multiplier * amplifying_reaction
}

fn forward_vape_multiplier(
	trigger: &CharStats
) -> f32 {
	2.0 * (1.0 + (2.78 * trigger.em) / (1400.0 + trigger.em) + trigger.reaction_bonus)
}

/////////////////////////////
////////// Shark ////////////
/////////////////////////////

pub fn shark_na_bite(
	shark: &CharStats,
	momentum: usize,
	vape: bool
) -> f32 {
	let mut wave_bonus = momentum as f32 * 0.078 * shark.hp;
	let na_multiplier = 15.6 / 100.0;
	let vape_multiplier = match vape {
		true => forward_vape_multiplier(&shark),
		false => 1.0
	};

	if momentum == 3 {
		wave_bonus += 0.391 * shark.hp;
	}

	damage(
		shark.hp * na_multiplier + shark.na_bonus_flat,
		1.0,
		wave_bonus,
		(shark.dmg_bonus + shark.na_bonus) / 100.0,
		shark.crit_rate,
		shark.crit_damage,
		shark.res_shred / 100.0,
		vape_multiplier
	)
}

pub fn shark_burst(
	shark: &CharStats,
	vape: bool
) -> f32 {
	let burst_multiplier = 105.2 / 100.0;
	let vape_multiplier = match vape {
		true => forward_vape_multiplier(&shark),
		false => 1.0
	};

	damage(
		shark.hp * burst_multiplier,
		1.0,
		0.0,
		(shark.dmg_bonus) / 100.0,
		shark.crit_rate,
		shark.crit_damage,
		shark.res_shred / 100.0,
		vape_multiplier
	)
}

/// Assuming 4 vaped normals, 3 of which are enhanced to max stacks.
/// Wrapped up with burst.
pub fn shark_vape(stats: &CharStats) -> f32 {
	// The duration of her skill seems to be around 6s idfk
	// Just assume she bites two times after applying 3 stacks each time
	let mut damage = 0.0;
	damage += shark_na_bite(&stats, 1, true);
	damage += shark_na_bite(&stats, 3, true);
	damage += shark_na_bite(&stats, 3, true);
	damage += shark_na_bite(&stats, 3, true);

	// Use the burst either as an opening move
	// or as a finisher, depending which is better
	let burst = shark_burst(&stats, true);
	damage += burst;
	damage
}

pub fn shark_furina_thoma_kazuha(
	mainstats: &[f32; 6],
	substats: &[usize; 5],
	base: impl Fn(CharStats) -> CharStats,
	buff: impl Fn(CharStats, CharStats) -> CharStats
) -> f32 {
	let stats1 = stats(
		characters::SHARK,
		&base,								// This is the weapon base stat function
		vec![								// This is a list of all the dynamic buffs
			&buff,
			&buffs::obsidian,
			&buffs::kazuha_e,
			&buffs::vv_shred,
			&buffs::thoma_c6,
			&buffs::hydro_resonance,
			&buffs::furina_burst(150.0),
			&buffs::scroll(false)				// Thoma is on scroll
		],
		mainstats,
		substats
	);

	shark_vape(&stats1)
}

pub fn shark_furina_thoma_nahida(
	mainstats: &[f32; 6],
	substats: &[usize; 5],
	base: impl Fn(CharStats) -> CharStats,
	buff: impl Fn(CharStats, CharStats) -> CharStats
) -> f32 {
	let stats = stats(
		characters::SHARK,
		&base,								// This is the weapon base stat function
		vec![								// This is a list of all the dynamic buffs
			&buff,
			&buffs::tenacity2pc,
			&buffs::hod2pc,
			&buffs::nahida_burst(800.0),
			&buffs::thoma_c6,
			&buffs::hydro_resonance,
			&buffs::furina_burst(150.0),
			&buffs::scroll(false),			// Furina is on scroll
			&buffs::instructor_share		// Thoma is on instructors
		],
		mainstats,
		substats
	);

	shark_vape(&stats)
}

pub fn shark_furina_sige_kazuha(
	mainstats: &[f32; 6],
	substats: &[usize; 5],
	base: impl Fn(CharStats) -> CharStats,
	buff: impl Fn(CharStats, CharStats) -> CharStats
) -> f32 {
	let stats = stats(
		characters::SHARK,
		&base,								// This is the weapon base stat function
		vec![								// This is a list of all the dynamic buffs
			&buff,
			&buffs::obsidian,
			&buffs::hydro_resonance,
			&buffs::furina_burst(200.0),
			&buffs::scroll(false),			// Sige is on scroll
			&buffs::kazuha_e,
			&buffs::vv_shred
		],
		mainstats,
		substats
	);

	shark_vape(&stats)
}

pub fn shark_yelan_xl_zhong(
	mainstats: &[f32; 6],
	substats: &[usize; 5],
	base: impl Fn(CharStats) -> CharStats,
	buff: impl Fn(CharStats, CharStats) -> CharStats
) -> f32 {
	let stats = stats(
		characters::SHARK,
		&base,								// This is the weapon base stat function
		vec![								// This is a list of all the dynamic buffs
			&buff,
			&buffs::tenacity2pc,
			&buffs::hod2pc,
			&buffs::yelan_a4,
			&buffs::hydro_resonance,
			&buffs::petra_share,			// Zhong is on petra
			&buffs::instructor_share		// Xiangling is on instructors
		],
		mainstats,
		substats
	);

	shark_vape(&stats)
}

/////////////////////////////
////////// Emilie ///////////
/////////////////////////////

fn emilie_lumidouce_case(char: &CharStats) -> f32 {
	let bonus = (char.atk / 1000.0).floor().min(3.0) * 0.15;
	let puff_multiplier = 151.2 / 100.0 * 2.0;

	let puff_damage = damage(
		char.atk * puff_multiplier,
		1.0,
		bonus,
		(char.dmg_bonus + char.skill_bonus) / 100.0,
		char.crit_rate,
		char.crit_damage,
		char.res_shred / 100.0,
		1.0
	);

	let cologne_multiplier = 600.0 / 100.0;
	let cologne_damage = damage(
		char.atk * cologne_multiplier,
		1.0,
		bonus,
		char.dmg_bonus / 100.0,
		char.crit_rate,
		char.crit_damage,
		char.res_shred / 100.0,
		1.0
	);

	puff_damage * 11.0 + cologne_damage * 9.0
}

pub fn emilie_furina_vape(
	mainstats: &[f32; 6],
	substats: &[usize; 5],
	base: impl Fn(CharStats) -> CharStats,
	buff: impl Fn(CharStats, CharStats) -> CharStats
) -> f32 {
	let stats = stats(
		characters::EMILIE,
		&base,								// This is the weapon base stat function
		vec![								// This is a list of all the dynamic buffs
			&buff,
			&buffs::reverie(5),
			&buffs::bennett_burst,
			&buffs::furina_burst(150.0)
		],
		mainstats,
		substats
	);

	emilie_lumidouce_case(&stats)
}