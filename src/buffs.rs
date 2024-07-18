use crate::CharStats;

/////////////////////////////
///////// WEAPONS ///////////
/////////////////////////////
pub fn soss_base(
	mut stats: CharStats,
) -> CharStats {
	stats.crit_rate += 44.1;
	stats.atk += 542.0;
	stats
}

pub fn soss_buff1(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.atk += 0.52 * stats.em;
	stats.atk += 0.28 * 1.0 * stats.em;
	stats
}

pub fn soss_buff2(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.atk += 0.52 * stats.em;
	stats.atk += 0.28 * 2.0 * stats.em;
	stats
}

pub fn soss_buff3(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.atk += 0.52 * stats.em;
	stats.atk += 0.28 * 3.0 * stats.em;
	stats
}

pub fn homa_base(
	mut stats: CharStats
) -> CharStats {
	stats.crit_damage += 66.4;
	stats.atk += 608.0;
	stats
}

pub fn homa_buff1(
	base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.hp += base.hp * 0.2;
	stats.atk += stats.hp * 0.008;
	stats
}

pub fn homa_buff2(
	base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.hp += base.hp * 0.2;
	stats.atk += stats.hp * 0.018;
	stats
}

/////////////////////////////
//////// CHARACTERS /////////
/////////////////////////////

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

pub fn mh2(
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
	stats = mh2(_base, stats);
	stats.crit_rate += 36.0;
	stats
}

pub fn shime2(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats.atk += 18.0;
	stats
}

pub fn shime(
	_base: CharStats,
	mut stats: CharStats
) -> CharStats {
	stats = shime2(_base, stats);
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