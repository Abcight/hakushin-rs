use crate::*;

pub fn weapon_calculator(
    weapons: Vec<(&str, &dyn Fn(CharStats) -> CharStats, Box<dyn Fn(CharStats, CharStats) -> CharStats>)>,
    arti_mainstat_distributions: Vec<[f32; 6]>,
    arti_substat_distributions: Vec<[usize; 5]>,
    rotation: fn(&[f32; 6], &[usize; 5], base: &dyn Fn(CharStats) -> CharStats, buff: &Box<dyn Fn(CharStats, CharStats) -> CharStats>) -> f32
) {
    let mut weapons_dpr = Vec::new();
	for (name, base, buff) in &weapons {
		let mut max_dpr = 0.0;
		for mainstats in &arti_mainstat_distributions {
			for substats in &arti_substat_distributions {
				let damage = rotation(mainstats, substats, base, buff);
				if damage > max_dpr {
					max_dpr = damage;
				}
			}
		}
		weapons_dpr.push((name, max_dpr));
	}

	weapons_dpr.sort_by_key(|x| x.1 as usize);
	weapons_dpr.reverse();
	let mut r1_damage = 0.0;
	for (name, dpr) in &weapons_dpr {
		if **name == "Surfing Time R1" {
			r1_damage = *dpr;
			break;
		}
	}

	println!(
		"{0: <23} {1: <13} {2: <10} Comments",
		"Weapon", "DPR", "% of R1"
	);
	for (name, dpr) in weapons_dpr {
		let of_r1 = (dpr / r1_damage * 100.0) as usize;
		println!(
			"{0: <23} {1: <13} {2: <11}",
			name, dpr as usize, of_r1
		);
	}
}