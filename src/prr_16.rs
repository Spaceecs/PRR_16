use itertools::Itertools;

#[test]
fn prr_16_test() {
    // Множина чисел від 1 до 8
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];

    // Генерація всіх перестановок 8 чисел
    for perm in digits.iter().permutations(8) {
        let (m, u, x, a, s, l, o, n) = (perm[0], perm[1], perm[2], perm[3], perm[4], perm[5], perm[6], perm[7]);

        // Обчислення чисел
        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;

        // Перевірка умови: muxa * a / slon == x
        if muxa * a == slon * x {
            println!("  {}", muxa);
            println!("{}        {}", x, a);
            println!("  ------");
            println!("   {}", slon);
        }
    }
}
