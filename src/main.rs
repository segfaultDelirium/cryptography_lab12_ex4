// zrobic do srody do 9 rano

fn resolve_symbol_legendre(symbol: (bool, i32, i32)) -> i32 {
    let (is_positive, a, p ) = symbol;
    let multiplier = if is_positive {1} else {-1};
    if modulo_euclid(a, p) == 0 {
        return 0;
    }
    if is_reszta_kwadratowa(a, p) {
        return multiplier * 1
    }
    multiplier * -1
}

fn resolve_jacobi_start(a: i32, n: i32) {

    let mut symbol = (true, a, n);
    while(true){
        symbol =  rule_1_substitute_or_return_arg(symbol);

    }

    println!();
}

fn resolve_jacobi(symbol: (bool, i32, i32)) -> i32 {
    let (is_positive, a, n ) = symbol;
    let mut symbol = (true, a, n);
    symbol =  rule_1_substitute_or_return_arg(symbol);

    1
}

fn main() {

    let a = 610;
    let n = 987;

    let m2 = find_m2(a, n);
    println!("m2 = {:?}", m2);

    // let rule_1_substitute_or_return_arg(a, n)

    // (is_positive, a, n)
    let mut symbol = (true, a, n);
    let mut res: i32;
    while(true){
        symbol =  rule_1_substitute_or_return_arg(symbol);
        break;
    }

    println!("Hello, world!");
}

fn find_m2(m1: i32, n: i32) -> Option<i32>{
    (1..m1).into_iter().find(|x| modulo_euclid(*x, n) == m1)
}

fn rule_1_substitute_or_return_arg(symbol: (bool, i32, i32)) -> (bool, i32, i32) {
    let (is_positive, a, n ) = symbol;
    let m2_option = find_m2(a, n);
    if let Some(m2) = m2_option{
        return (is_positive, m2, n)
    }

    (is_positive, a, n)
    // (0, 0)
}

fn rule_2(n: i32) -> i32 {
    let n_modulo_8 = modulo_euclid(n, 8).abs();
    if n_modulo_8 == 1 {
        return 1
    }

    if n_modulo_8 == 3{
        return -1
    }
    0
}

// fn rule3(n: i32) ->  ((i32, i32), (i32, i32)){
//
// }


// fn resolve_legendre(a: i32, p: i32) -> i32{
//     if modulo_euclid(a, p) == 0 {
//         return 0;
//     }
//     if is_reszta_kwadratowa(a, p) {
//         return 1
//     }
//     -1
// }

fn is_reszta_kwadratowa(a: i32, p: i32) -> bool {
    modulo_euclid(a.pow(((p - 1) / 2) as u32), p) == 1
}

fn modulo_euclid(j: i32, k: i32) -> i32 {
    let res =  j % k;
    if res < 0 {return res + k} else {return res}
}
