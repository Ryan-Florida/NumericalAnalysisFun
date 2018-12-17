//ROWS -> rows in grid.
pub const ROWS: usize = 101;
//COLS -> columns in grid.
pub const COLS: usize = 11;
//H -> Length (in meters) of linear segments.
pub const H: f64 = 1e-1;
//K -> Time step (in seconds) after calculations start.
pub const K: f64 = 1e-3;

//This function applies specified boundary conditions to input array.
pub fn apply_boundary_conditions(u: &mut [[f64; COLS]; ROWS])
{
    for i in 0usize..ROWS
    {
        u[i][0] = 0f64;
        u[i][COLS - 1] = 0f64;
    }
}

//This function applies specified initial conditions to input array.
pub fn apply_initial_conditions(u: &mut [[f64; COLS]; ROWS])
{
    for i in 1usize..COLS/2usize
    {
        u[0][i] = 2f64*(i as f64)*H;
    }
    for i in COLS/2usize..COLS
    {
        u[0][i] = 2f64*(1f64 - (i as f64)*H);
    }
}
