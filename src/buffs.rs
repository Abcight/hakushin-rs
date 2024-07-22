use crate::CharStats;

/////////////////////////////
///////// WEAPONS ///////////
/////////////////////////////

pub fn sac_jade_base(
	mut stats: CharStats,
) -> CharStats {
	stats.crit_rate += 36.8;
	stats.atk += 454.0;
	stats
}

pub fn sac_jade_buff(
	refinement: usize,
) -> impl Fn(CharStats, CharStats) -> CharStats {
	assert!(refinement >= 1);
	assert!(refinement <= 5);
	move |base, mut stats| {
		stats.hp += base.hp * (24.0 + refinement as f32 * 8.0) / 100.0;
		stats.em = 30.0 + refinement as f32 * 10.0;
		stats
	}
}

pub fn soss_base(
	mut stats: CharStats,
) -> CharStats {
	stats.crit_rate += 44.1;
	stats.atk += 542.0;
	stats
}

pub fn soss_buff(
	stacks: usize
) -> impl Fn(CharStats, CharStats) -> CharStats {
	move |_, mut stats| {
		stats.atk += 0.52 * stats.em;
		stats.atk += 0.28 * stacks as f32 * stats.em;
		stats
	}
}

pub fn homa_base(
	mut stats: CharStats
) -> CharStats {
	stats.crit_damage += 66.4;
	stats.atk += 608.0;
	stats
}

pub fn homa_buff(
	under_half_hp: bool
) -> impl Fn(CharStats, CharStats) -> CharStats {
	move |base, mut stats| {
		stats.hp += base.hp * 0.2;
		stats.atk += match under_half_hp {
			true => stats.hp * 0.018,
			false => stats.hp * 0.008
		};
		stats
	}
}

pub fn surfing_time_base(
	mut base: CharStats,
) -> CharStats {
	base.crit_damage += 88.2;
	base.atk += 542.0;
	base
}

pub fn surfing_time_buff(
	refinement: usize,
	stacks: usize
) -> impl Fn(CharStats, CharStats) -> CharStats {
	assert!(refinement >= 1);
	assert!(refinement <= 5);
	assert!(stacks <= 4);
	let hp_boost = 15.0 + 5.0 * refinement as f32;	// 20% R1, 5% per
	let na_stack = 9.0 + 3.0 * refinement as f32;	// 12% R1, 3% per
	move |base, mut stats| {
		stats.hp += hp_boost / 100.0 * base.hp;
		stats.na_bonus += na_stack * stacks as f32;
		stats
	}
}

pub fn solar_pearl_base(
	mut base: CharStats,
) -> CharStats {
	base.crit_rate += 27.6;
	base.atk += 510.0;
	base
}

pub fn solar_pearl_buff(
	refinement: usize
) -> impl Fn(CharStats, CharStats) -> CharStats {
	assert!(refinement >= 1);
	assert!(refinement <= 5);
	move |_, mut stats| {
		stats.na_bonus += 15.0 + 5.0 * refinement as f32;
		stats
	}
}

pub fn tulaytullah_base(
	mut base: CharStats,
) -> CharStats {
	base.crit_damage += 44.1;
	base.atk += 674.0;
	base
}

pub fn tulaytullah_buff(
	refinement: usize,
	stacks_time: usize,
	stacks_hit: usize
) -> impl Fn(CharStats, CharStats) -> CharStats {
	assert!(refinement >= 1);
	assert!(refinement <= 5);
	let na_stack = 3.6 + refinement as f32 * 1.2;	// 4.8 at R1, 1.2 per
	let na_hit = 7.2 + refinement as f32 * 2.4;		// 9.6 at R1, 2.4 per
	move |_, mut stats| {
		stats.na_bonus += na_stack * stacks_time as f32;
		stats.na_bonus += na_hit * stacks_hit as f32;
		stats
	}
}

pub fn prayer_base(
	mut base: CharStats,
) -> CharStats {
	base.crit_rate += 33.1;
	base.atk += 608.0;
	base
}

pub fn prayer_buff(
	refinement: usize,
	stacks: usize
) -> impl Fn(CharStats, CharStats) -> CharStats {
	assert!(refinement >= 1);
	assert!(refinement <= 5);
	move |_, mut stats| {
		stats.dmg_bonus += (6.0 + 2.0 * refinement as f32) * stacks as f32;
		stats
	}
}

pub fn ceiba_base(
	mut base: CharStats,
) -> CharStats {
	base.hp += 0.413 * base.hp;
	base.atk += 510.0;
	base
}

pub fn ceiba_buff(
	refinement: usize
) -> impl Fn(CharStats, CharStats) -> CharStats {
	assert!(refinement >= 1);
	assert!(refinement <= 5);
	move |base, mut stats| {
		let max_increase = 12.0 + 4.0 * refinement as f32;
		let increase = 5.0 * (base.hp / 1000.0); // 5% per every 1000hp
		stats.na_bonus += increase.clamp(0.0, max_increase);
		stats
	}
}

/////////////////////////////
//////// CHARACTERS /////////
/////////////////////////////

pub fn shark_c1(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.constellation += 1;
	stats
}

pub fn nahida_burst(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.em += 200.0; // 800 em Nahida
	stats
}

pub fn zhong_shred(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.res_shred += 20.0;
	stats
}

pub fn tao_skill(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.atk += stats.hp * 0.0626;
	stats
}

pub fn tao_a4(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.dmg_bonus += 33.0;
	stats
}

pub fn bennett_burst(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.atk += 1000.0;
	stats.dmg_bonus += 15.0;
	stats
}

pub fn furina_burst(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.dmg_bonus += 75.0;
	stats
}

pub fn yelan_a4(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.dmg_bonus += 25.0; // averaged out
	stats
}

pub fn pyro_resonance(
	base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.atk += base.atk * 0.25;
	stats
}

pub fn hydro_resonance(
	base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.hp += base.hp * 0.25;
	stats
}

/////////////////////////////
//////// Artifacts //////////
/////////////////////////////

pub fn hod2pc(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.dmg_bonus += 15.0;
	stats
}

pub fn hod(
	base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats = hod2pc(base, stats);
	stats.na_bonus += 30.0;
	stats.ca_bonus += 30.0;
	stats
}

pub fn bollide(
	base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.na_bonus += 40.0;
	stats.ca_bonus += 40.0;
	stats
}

pub fn instructor2(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.em += 80.0;
	stats
}

pub fn instructor_share(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.em += 120.0;
	stats
}

pub fn mh2pc(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.na_bonus += 15.0;
	stats.ca_bonus += 15.0;
	stats
}

pub fn mh(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats = mh2pc(_base, stats);
	stats.crit_rate += 36.0;
	stats
}

pub fn shime2pc(
	base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.atk += 0.18 * base.atk;
	stats
}

pub fn shime(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats = shime2pc(_base, stats);
	stats.na_bonus += 50.0;
	stats
}

pub fn mhplus(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.crit_rate += 40.0;
	stats.dmg_bonus += 20.0;
	stats
}

pub fn scrl(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.dmg_bonus += 45.0;
	stats
}

pub fn petra_share(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.dmg_bonus += 35.0;
	stats
}