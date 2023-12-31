use plotters::prelude::*;

fn main() {
    
  let root_drawing_area = BitMapBackend::new("images/1.png", (600, 400))
    .into_drawing_area();

  root_drawing_area.fill(&WHITE).unwrap();

  let mut ctx = ChartBuilder::on(&root_drawing_area)
    .build_cartesian_2d(0..100, 0..100)
    .unwrap();

  ctx.configure_mesh().draw().unwrap();

}
