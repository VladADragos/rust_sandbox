
pub fn test_mat(){

    let mat:[[f64;3];3] = [
        [1.0,2.0,3.0],
        [4.0,5.0,6.0],
        [7.0,8.0,9.0]
        ];
    
    assert_eq!(det(&mat),1.0);
}

            

fn det(mat: &[[f64;3];3])->f64{
    // let a = mat[0][0];
    // let b = mat[0][1];
    // let c = mat[0][2];
    // let d = mat[1][0];
    // let e = mat[1][1];
    // let f = mat[1][2];
    // let g = mat[2][0];
    // let h = mat[2][1];
    // let i = mat[2][2];
return 
mat[0][0]*mat[1][1]*mat[2][2]+
mat[0][1]*mat[1][2]*mat[2][0]+
mat[0][2]*mat[1][0]*mat[2][1]-
mat[0][0]*mat[1][2]*mat[2][1]-
mat[0][1]*mat[1][0]*mat[2][2]-
mat[0][2]*mat[1][1]*mat[2][0]
}
