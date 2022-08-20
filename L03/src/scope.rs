pub(crate) fn scope() {
    let valor_x = 20;
    {
        let valor_x = 15;
        println!("{}", valor_x)
    }
    println!("{}", valor_x)
}
