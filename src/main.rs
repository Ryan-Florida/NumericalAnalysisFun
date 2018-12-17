extern crate example;
//*****************ONLY ATTEMPT TO RUN ONE EXAMPLE AT A TIME.******************

/*Uncomment line below to run example1.*/
use example::example1::*;

/*Uncomment line below to run example2.*/
//use example::example2::*;

/*Uncomment line below to run example3.*/
//use example::example3::*;

/*Uncomment line below to run example4.*/
//use example::example4::*;

//This function numerically solves the selected example using the method of
//finite differences.
fn numerically_solve(u: &mut [[f64; COLS]; ROWS])
{
    let r: f64 = K/(H*H);
    for j in 0usize..ROWS - 1usize
    {
        for i in 1usize..COLS - 1usize
        {
            u[j + 1usize][i] = r*u[j][i - 1usize] + (1f64 - 2f64*r)*u[j][i]
                               + r*u[j][i + 1usize];
        }
    }
}

//This function simply displays the results in a nicely formatted way.
//**IF THE FORMATTING BECOMES INCREASINGLY BROKEN, THAT SHOULD BE A SIGN THAT
//YOU ARE DEALING WITH A SYSTEM THAT IS VERY UNSTABLE.**
fn display_results(u: [[f64; COLS]; ROWS])
{
    for i in 0usize..ROWS
    {
        for j in 0usize..COLS
        {
            print!("  {:.4}   ", u[i][j]);
        }
        println!();
    }
}

//This is just the driver-function.
fn main()
{
    //u will represent our grid. All the examples above refer to a uniform metal
    //bar experiencing heat diffusion. Each column of u represents the
    //temperature of a given segment of the bar. Each row of u represents the
    //entire bar at the given timestep. The segments lengths and the timesteps
    //are to be set in the example files.
    let mut u: [[f64; COLS]; ROWS] = [[0f64; COLS]; ROWS];
    apply_boundary_conditions(&mut u);
    apply_initial_conditions(&mut u);
    numerically_solve(&mut u);
    display_results(u);
}

