fn main() {
    let mut data = [-5, 0, -5, 19, -27, 3, -4, -5, 32];

    for x in 0..data.len() {
        let mut smallest_index = x;
        for y in x..data.len() {
            if data[y] < data[smallest_index] {
                smallest_index = y;
            }
        }

        let buffer = data[x];
        data[x] = data[smallest_index];
        data[smallest_index] = buffer;
    }

    println!("{:?}", data);
}
