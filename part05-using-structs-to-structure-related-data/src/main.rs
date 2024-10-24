use std::fmt;

fn main() {

   #[derive(Debug)]
   struct User {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64
   }

   let user1 = User {
      active: true,
      username: String::from("MrDoyle"),
      email: String::from("something@me.com"),
      sign_in_count: 1
   };

   let mut user2 = User {
      active: true,
      username: String::from("MrDoyle"),
      email: String::from("something@me.com"),
      sign_in_count: 1
   };

   user2.email = String::from("deez@nutz.com");

   //Cannot be the other way around or else it fails
   let user3 = User {
      email: String::from("user@three.com"),
       ..user2
   };

   println!("{:?}", user2.active);
   println!("{:?}", user3);

   //If all fields of a struct are Copy then the strict itself is also Copy
   #[derive(Debug)]
   struct Blah {
      test: bool,
      something: u32
   }

   let y = Blah{test: false, something: 42};
   let x = Blah{something: 50, ..y};

   println!("{}", y.test);
   println!("{:?}", y);


   #[derive(PartialEq)]
   struct AlwaysEqual;

   let subject1 = AlwaysEqual;
   let subject2 = AlwaysEqual;

   let isEqual: bool = subject1 == subject2;
   println!("{}", isEqual);

   // An example of a program using structs

   let width1: u32 = 30;
   let height1: u32 = 5;

   println!("The area of the rectangle is {} square pixels", area(width1, height1));

   fn area(width: u32, height: u32) -> u32 {
      width * height
   }

   let rect1:(u32, u32) = (30, 50);

   println!("the area of the rectangle is {} square pixels", area2(rect1));

   fn area2(dimensions: (u32, u32)) -> u32 {
      dimensions.0 * dimensions.1
   }

   #[derive(Debug)]
   struct Rectangle {
      width: u32,
      height: u32,
   }

   let rect2 = Rectangle {
      width: 30,
      height: 50
   };

   println!("the area of the rectangle is {} square pixels", area3(&rect2));

   fn area3(rectangle: &Rectangle) -> u32 {
      rectangle.width * rectangle.height
   }

   println!("rect1 is {rect1:#?}");
   dbg!(&rect1);

   let scale = 2;
   let rect4 = Rectangle{
      width: dbg!(30 * scale),
      height: 42
    };

   dbg!(&rect4);

   impl Rectangle {
      fn area(&self) -> u32 {
         self.height * self.width
      }
      fn width(&self) -> bool {
         self.width > 0
      }
      fn can_hold(&self, other: &Rectangle) -> bool {
         self.width > other.width && self.height > other.height
      }
      fn square(size: u32) -> Self {
         Self {
            width: size,
            height: size,
         }
      }
   }

   if rect4.width() {
      println!("The rectangle has a nonzero width; it is {}", rect4.width);
   }

   let rect5 = Rectangle {
      width: 30,
      height: 50,
   };
   let rect6 = Rectangle {
      width: 10,
      height: 40,
   };
   let rect7 = Rectangle {
      width: 60,
      height: 45,
   };

   println!("Can rect1 hold rect2? {}", rect5.can_hold(&rect6));
   println!("Can rect1 hold rect3? {}", rect5.can_hold(&rect7));

   let sq = Rectangle::square(3);
   println!("square is {:?}", sq); // https://stackoverflow.com/a/38157410
}
