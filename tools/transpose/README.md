# Transpose Generator

Generate transpose index used in the main **gomoku** binary.

## Usage

Comment/uncomment all constants you need to generate, e.g for the diagonal:

```rust
let transpose = generate_swap_diagonal();
display_swap_diagonal(&transpose);
display_diagonal_window_five_slices(&transpose);
```

And save the output to the file `/src/transpose.rs`.
