use std::rc::Rc;

#[derive(Debug)]
struct Employee {
  name: String,
}

// #[derive(Debug)]
// struct Company {
//   name: String,
//   manager: Employee,
// }

// todo: make the lifecycle correct, this can be compile pass, but not godd style
#[derive(Debug)]
struct Company<'a> {
  name: String,
  manager: &'a Employee,
}

pub fn comm() {
  /*
  let employee = Employee {
    name: "Jay".to_string()
  };

  let company1 = Company {
    name: "company 1".to_string(),
    manager: employee
  };

  let company2 = Company {
    name: "company 2".to_string(),
    manager: employee     // todo: if dont use Rc, compile wrong!
  };
   */

  // +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

  // todo: below style two company can not get the ownership of the `manager` attribute
  /*
  let employee = &Employee {
    name: "Jay".to_string()
  };

  let company1 = Company {
    name: "company 1".to_string(),
    manager: employee,
  };

  let company2 = Company {
    name: "company 2".to_string(),
    manager: employee,
  };

  println!("{:?}, {:?}", company1, company2);
   */


  // todo: enable more owner for `manager`, can has ownership at the same time
  let x = 9;
  let xx = x.clone();       // todo: will clone the inner data, will add the memory for store new data

  let y = Rc::new(99);
  let yy = y.clone();    // todo: not clone the inner data, just add reference counter

  let num = Rc::new(5);
  println!("{}", Rc::strong_count(&num)); // 1, get the reference count of Rc
  let num1 = num.clone();   // todo: when call Rc.clone(), just copy struct of Rc in  `stack`, create a new Rc struct in `stack`, and update the reference counter in heap
  {
    println!("{}", Rc::strong_count(&num1)); // 2
    let num2 = num.clone();
    println!("{}", Rc::strong_count(&num2)); // 3
  }
  println!("{}", Rc::strong_count(&num)); // 2

  // +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
  // todo: correct style to use Rc
  #[derive(Debug)]
  struct Employee {
    name: String,
  }

  #[derive(Debug)]
  struct Company {
    name: String,
    manager: Rc<Employee>,
  }

  let employee = Rc::new(Employee {
    name: "Jay".to_string()
  });
  
  let company1 = Company {
    name: "company 1".to_string(),
    manager: employee.clone()
  };

  let company2 = Company {
    name: "company 2".to_string(),
    manager: employee.clone()
  };

  println!("{:?}, {:?}", company1, company2);
}











