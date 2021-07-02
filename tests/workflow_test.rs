use leetcode_picker::*;

#[test]
fn work_flow_test() {
    let resp = match Quiz::get_by_name("capacity-to-ship-packages-within-d-days") {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            panic!();
        }
    };

    // test if case in disk as same as online version
    let content = include_str!("./questions_description_test_case1");
    assert_eq!(resp.quiz_pure_description().unwrap(), content);

    assert_eq!(resp.quiz_id().unwrap(), "1011");
    assert_eq!(
        resp.quiz_source(),
        "https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/"
    );
}

#[test]
fn pick_by_id_work_flow_test() {
    let resp = Quiz::get_by_id(1011).unwrap();
    //dbg!(&resp);
    assert_eq!(resp.quiz_id().unwrap(), "1011");
}

#[test]
fn pick_by_level_work_flow_test() {
    let resp = Quiz::get_randomly(Some(Level::Easy)).unwrap();
    assert_eq!(resp.quiz_level(), &Level::Easy);
}
