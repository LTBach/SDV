fn main() {
    let objetive = 3126.59;

    // 27
    let values: Vec<f64> = vec![
        2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
        139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
        4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
    ];

    let values_number = values.len(); //27
    let values_index_max = values_number - 1; //26

    let mut additions: Vec<usize> = vec![0];

    println!("{:?}", values_number); //1

    
    while additions.len() > 0 {
        let mut addition: f64 = 0.0;
        let mut saltar: i32 = 0;

        // Sumar valores en additions
        for element_index in additions.iter() {
            let addition_aux = values[*element_index];
            addition = addition_aux + addition;
        }
    }
}