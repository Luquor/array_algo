fn main() {
    let numbers: [i32; 100] = [
      812, 476, -135, 993, 58, 631, 417, 205, -42, 904,
      337, 788, 129, 552, 682, 416, -87, 949, 271, 705,
      564, 873, 492, 118, 305, 640, 721, 867, 430, -196,
      958, 284, 613, 744, 199, 532, 381, 827, 902, 147,
      -63, 511, 674, 388, 596, 248, 472, 905, 334, 781,
      146, 664, -214, 873, 520, 437, 792, 365, 688, -51,
      998, 289, 577, 806, 254, 612, 349, 934, 150, 478,
      763, 224, 571, 856, 432, 699, 118, -102, 985, 310,
      547, 903, 206, 752, 498, 377, 845, 268, 639, 721,
      401, 874, 133, 566, 918, 247, -188, 664, 292, 807,
    ];

    println!("---   ---   ---   ---   ---   ---   ---   ---   ---   ---   ---   ---    ---");
    println!("---     This program will do some calculation on the following array     ---");
    println!("---                    (e.g. min, max, average, sort)                    ---");
    println!("---   ---   ---   ---   ---   ---   ---   ---   ---   ---   ---   ---    ---");
    println!();

    let lowest = find_min(&numbers);
    let highest = find_max(&numbers);
    let average = calculate_average(&numbers);

    println!("Numbers: {:?}", numbers);
    println!("Lowest value: {}", lowest);
    println!("Highest value: {}", highest);
    println!("Average value: {}", average);

    let sorted_numbers = sort_numbers(&numbers);
    println!("Sorted values: {:?}", sorted_numbers);

}

fn find_min(arr: &[i32]) -> i32 {
    let mut lowest = arr[0];
    for &value in arr {
        if value < lowest {
            lowest = value;
        }
    }
    return lowest;
}

fn find_max(arr: &[i32]) -> i32 {
    let mut highest = arr[0];
    for &value in arr {
        if value > highest {
            highest = value;
        }
    }
    return highest;
}

fn calculate_average(arr: &[i32]) -> f64 {
    let mut sum = 0;
    for &value in arr {
        sum += value;
    }
    return sum as f64 / arr.len() as f64;
}

fn sort_numbers(arr: &[i32]) -> Vec<i32> {
    let mut sorted = arr.to_vec();

    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 0..sorted.len() - 1 {
            if sorted[i] > sorted[i + 1] {
                let tmp = sorted[i];
                sorted[i] = sorted[i + 1];
                sorted[i + 1] = tmp;
                swapped = true;
            }
        }
    }

    sorted
}
