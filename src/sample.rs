#[allow(dead_code)]
pub fn small_start(w: usize, h: usize) -> Vec<Vec<bool>> {
    debug!("creating grid of {}x{}", w, h);
    let mut v = vec![vec![false; h]; w];
    // debug!("created grid of {:?}", v);
    v[w / 2 - 1][h / 2] = true;
    v[w / 2][h / 2 + 1] = true;
    v
}

#[allow(dead_code)]
pub fn clustering(w: usize, h: usize) -> Vec<Vec<bool>> {
    debug!("creating grid of {}x{}", w, h);
    let mut v = vec![vec![false; h]; w];

    v[w / 2 - 1][h / 2] = true;
    v[w / 2][h / 2 + 1] = true;
    v[w / 2][h / 2 - 1] = true;
    v[w / 2 + 1][h / 2] = true;
    v
}

#[allow(dead_code)]
pub fn seed1(w: usize, h: usize) -> Vec<Vec<bool>> {
    debug!("creating grid of {}x{}", w, h);
    let mut v = vec![vec![false; h]; w];

    v[1][1] = true;
    v[1][2] = true;
    v[1][3] = true;
    v[6][1] = true;
    v[6][2] = true;
    v[7][1] = true;
    v[7][2] = true;
    v[w / 2 - 1][h / 2] = true;
    v[w / 2][h / 2 + 1] = true;
    v[w / 2][h / 2] = true;
    v[w / 2][h / 2 - 1] = true;
    v[w / 2 + 1][h / 2] = true;
    v
}

#[allow(dead_code)]
pub fn seed2(w: usize, h: usize) -> Vec<Vec<bool>> {
    debug!("creating grid of {}x{}", w, h);
    let mut v = vec![vec![false; h]; w];

    v[6][6] = true;
    v[7][6] = true;
    v[8][6] = true;
    v[10][6] = true;

    v[6][7] = true;

    v[9][8] = true;
    v[10][8] = true;

    v[7][9] = true;
    v[8][9] = true;
    v[10][9] = true;

    v[6][10] = true;
    v[8][10] = true;
    v[10][10] = true;

    v
}
