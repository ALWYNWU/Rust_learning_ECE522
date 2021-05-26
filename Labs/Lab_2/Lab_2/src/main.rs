fn main(){
    let mut groups = [[""; 4]; 6];
    groups[0]=["Bob", "Carol", "Eric", "Matt"];
    groups[1]=["Jim", "Lucy", "Terry", "Brenda"];
    groups[2]=["Susan", "Brad", "Jim", "Matt"];
    groups[3]=["Sue", "Wendy", "Sam", "Brad"];
    groups[4]=["Kate", "Jack", "James", "Sydney"];
    groups[5]=["Mary", "John", "Ricky", "Wendy"];

    searchMember("John",&mut groups)
}

fn searchMember(name: &str, list: &mut [[&str; 4]; 6]) {

    let mut exist = 0;
    let mut leader = 0;
    let mut number: Vec<usize> = Vec::new();

    for i in 0..6{
        for j in 0..4{
            if name == list[i][j] {
                exist = 1;
                number.push(i);
            }
            if name == list[i][0] {
                leader = 1;
            }
        }
    }

    if exist == 1 {
        println!("Yes");

        println!("The group number is:");
        for i in &number {
            println!("{}", i);
        }

    }
    else {
        println!("No");
    }


    if leader == 1 {
        println!("It is a group leader");
    }
    else {
        println!("It is not a group leader");
    }


}
