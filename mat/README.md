# Mat
Mat is a matrix library made for rnn

## Reference

- Creating a new matrix
```rs
// Create 2x3 matrix filled with 0
let my_mat = mat![0; 2, 3];

// Create 2x3 matrix with different values
let my_mat = mat![
    1, 2, 3;
    4, 5, 6;
];

// Create one row matrix with diferent values
let my_mat = mat![row 1, 2]; // 1x2 matrix

// Create one column matrix with diferent values
let my_mat = mat![col 1, 2]; // 2x1 matrix
```

- Iterating an matrix
```rs
let my_mat = mat![
    1, 2, 3;
    4, 5, 6;
];

for (i, j) in my_mat.iter() {
    if j == 0 { println!(); }
    print!("{} ", my_mat.data[i * my_mat.cols + j])
}
```

- Get value of matrix
```rs
let my_mat = mat![
    1, 2, 3;
    4, 5, 6;
];

let (i, j) = (0, 0);
println!("{}", my_mat[(i, j)]);
```

- Matrix operations
```rs
let a = mat![0; 2, 3];
let b = mat![0; 2, 3];

let c = a + b; // add mat (also accept +=);
let c = a + 1; // add mat with scalar (also accept +=);

let c = a - b; // sub mat (also accept -=);
let c = a - 1; // sub mat with scalar (also accept -=);

let a = mat![0; 2, 3];
let b = mat![0; 3, 2];

let c = a * b; // mul mat (does not accept *=)
```
