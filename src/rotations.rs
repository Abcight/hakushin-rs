use crate::CharStats;

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
		true => (0.5, 0.9),
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
		shark.res_shred / 100.0,
		vape_multiplier
	)
}

fn shark_burst(
	shark: &CharStats,
	vape: bool
) -> f32 {
	let burst_multiplier = 75.7 / 100.0;
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

/// Assuming 3 vaped normals, 2 of which are enhanced to max stacks.
/// Wrapped up with burst.
pub fn shark_n3_vape(stats: &CharStats) -> f32 {
    // The duration of her skill seems to be around 6s idfk
    // Just assume she bites two times after applying 3 stacks each time
    let mut damage = 0.0;
    damage += shark_na_bite(&stats, 3, true);
    damage += shark_na_bite(&stats, 3, true);

	// We can skill twice in a rotation
	damage += shark_na_bite(&stats, 3, true);
    damage += shark_na_bite(&stats, 3, true);

    // Finish off with her burst, vaped
    damage += shark_burst(&stats, true);
    damage
}