///
///
/// just rust hello world
///
///

fn main() {
    test1_data_type();
    test2_function();
    test3_condition();
    test4_while();
    test5_slice();
    test6_structure();
    test7_enum();
}

fn test1_data_type() {
    println!("Hello, world!");

    let a = 123; //不可变变量
    println!("oringe: a = {}!", a);

    let mut b = 234; //可变变量
    println!("b is {}", b);
    b = 456;
    println!("b is {}", b);

    //常量，不能被重新绑定。
    const E: i32 = 123;
    println!("E is {}", E);

    let a = 123; //变量的值可以重新绑定， 但是，在重新绑定之前不能私自被改变。
    println!("modify: a = {}!", a);

    let c: u64 = 123; //可以声明类型，如果这里自动推导，则为32位变量。
    println!("c is {}", c);

    /*
     * 重影 shadowing：指变量的名称可以被重新使用的机制。
     * 重影和可变变量的赋值不是一个概念，重影是指用同一个名字重新代表另一个变量实体，其类型，可变属性和值都可以变化。
     * 但是，可变变量仅能发生值的变化。
     */
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    //数据类型
    let g = 2.0;
    let h: f32 = 3.0;
    println!("g = {}, h = {}", g, h);

    let m = 98_222;
    println!("m = {}", m);

    let m = 0xff;
    println!("m = {}", m);

    let m = 0o77;
    println!("m = {}", m);

    let m = 0b1111_0000;
    println!("m = {}", m);

    let m = b'A';
    println!("m = {}", m);

    let m = false;
    println!("m = {}", m);

    let m = "aaa";
    println!("m = {}", m);

    let tup = (500, 6.4, 1);
    let (_x, _y, _z) = tup;

    let _m = [1, 2, 3];
    let _m = ["January", "February", "March"];
    let _m: [i32; 5] = [1, 2, 3, 4, 5]; //m是一个长度为5的i32数组
    let _m = [3; 5]; //等同于let d = [3,3,3,3,3];
    let _first = _m[0];

    //m[0] = 123; //错误， 数组m是不可变
    let mut m = [1, 2, 3];
    m[0] = 4;
}

fn another_function(x: i32, y: i32) {
    println!("x = {}", x);
    println!("y = {}", y);
}

fn test2_function() {
    let x = 5;

    //表达式
    let y = {
        let x = 3;
        x + 1
    };

    println!("x = {}", x);
    println!("y = {}", y);

    another_function(1,2);
    test2();
    println!("1 + 2 = {}", add(1, 2));
}

//函数嵌套
fn test2() {
    //注意->用来声明函数返回值的类型
    fn five() -> i32 {
        5
    }
    println!("five() = {}", five());
}

/* 注意： Rust不支持返回值类型判断！
 * 如果没有明确声明函数返回值的类型，函数将被认为是“纯过程”，不允许产生返回值，return后面不能有返回值表达式。
 * 目的是让公开的函数能够形成可见的公报。
 * 函数体表达式并不能等同于函数体，它不能使用return关键字。
 */
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

//条件控制
fn test3_condition() {
    let a = 12;
    let b;

    /*
     * if的条件不需要加小括号；
     * if的分支必须加大括号；
     * if的条件必须是bool类型。
     * 注意：c中的非0为真的概念在rust中不适用！！！
     */
    if a > 0 {
        b = 1;
    } else if a < 0 {
        b = -1;
    } else {
        b = 0;
    }

    println!("b = {}", b);

    test4();
}

fn test4() {
    let a = 3;

    //类似于函数体表达式
    //注意：两个函数体表达式的类型必须一样！且必须有一个 else 及其后的表达式块。
    let number = if a > 0 { 1 } else { -1 };
    println!("number = {}", number);
}

// 循环控制
fn test4_while() {
    let mut number = 1;

    //注意：rust中没有c语言中for循环使用的三元语句控制循环，可用while来代替。
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT");

    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("value = {}", i);
    }

    let a = [10, 20, 30, 40, 50];
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }

    //某个循环无法在开头和结尾判断是否继续进行循环，必须在循环体中间某处控制循环的进行。
    //如果遇到这种情况，我们经常会在一个 while (true) 循环体里实现中途退出循环的操作。
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'O' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }

    //loop 循环可以通过 break 关键字类似于 return 一样使整个循环退出并给予外部一个返回值。
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    let location = loop {
        let ch = s[i];
        if ch == 'O' {
            break i;
        }
        i += 1;
    };
    println!(" \'O\' index = {}", location);
}

fn test5_slice() {
    let s = String::from("broadcase");

    /* Rust 中的字符串类型实质上记录了字符在内存中的起始位置和其长度
     * 使用 .. 表示范围的语法在循环章节中出现过。x..y 表示 [x, y) 的数学含义。
     * 被切片引用的字符串禁止更改其值。
     *
     * 在 Rust 中有两种常用的字符串类型：str 和 String。
     * str 是 Rust 核心语言类型，就是本章一直在讲的字符串切片（String Slice），常常以引用的形式出现（&str）。
     * 凡是用双引号包括的字符串常量整体的类型性质都是 &str。
     *
     * String 类型是 Rust 标准公共库提供的一种数据类型，它的功能更完善——它支持字符串的追加、清空等实用的操作。
     * String 和 str 除了同样拥有一个字符开始位置属性和一个字符串长度属性以外还有一个容量（capacity）属性。
     * String 和 str 都支持切片，切片的结果是 &str 类型的数据。
     * 注意：切片结果必须是引用类型，但开发者必须自己明示这一点:
     */
    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{} + {} = {}", part1, part2, s);

    let s = "hello";
    let slice = &s[0..3];
    println!("{}, {}", s, slice);

    let s1 = String::from("hello");
    let s2 = &s1[..];
    println!("{}, {}", s1, s2);

    //数组切片
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter() {
        println!("{}", i);
    }
}

/*
 * Rust 中的结构体（Struct）与元组（Tuple）都可以将若干个类型不一定相同的数据捆绑在一起形成整体，
 * 但结构体的每个成员和其本身都有一个名字，这样访问它成员的时候就不用记住下标了。
 * 元组常用于非定义的多值传递，而结构体用于规范常用的数据结构。结构体的每个成员叫做"字段"。
 *
 * 注意：如果你常用 C/C++，请记住在 Rust 里 struct 语句仅用来定义，不能声明实例，结尾不需要 ; 符号，而且每个字段定义之后用 , 分隔。
 */
#[derive(Debug)]
struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32,
}

//结构体方法
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//请注意，在调用结构体方法的时候不需要填写 self ，这是出于对使用方便性的考虑。
//贴士：结构体 impl 块可以写几次，效果相当于它们内容的拼接！
impl Rectangle {
    //结构体方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //结构体函数
    //之所以"结构体方法"不叫"结构体函数"是因为"函数"这个名字留给了这种函数：它在 impl 块中却没有 &self 参数。
    //这种函数不依赖实例，但是使用它需要声明是在哪个 impl 块中的。
    //一直使用的 String::from 函数就是一个"关联函数"。
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn test6_structure() {
    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013,
    };
    println!("runoob is {:?}", runoob);

    //如果正在实例化的结构体有字段名称和现存变量名称一样的，可以简化书写：
    let domain = String::from("www.runoob.com");
    let name = String::from("RUNOOB");
    let runoob = Site {
        domain, // 等同于 domain : domain,
        name,   // 等同于 name : name,
        nation: String::from("China"),
        found: 2013,
    };
    println!("runoob is {:?}", runoob);

    //你想要新建一个结构体的实例，其中大部分属性需要被设置成与现存的一个结构体属性一样，仅需更改其中的一两个字段的值，可以使用结构体更新语法。
    //注意：..runoob 后面不可以有逗号。这种语法不允许一成不变的复制另一个结构体实例，意思就是说至少重新设定一个字段的值才能引用其他实例的值。
    let site = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        ..runoob
    };
    println!("site is {:?}", site);

    //元组结构体
    //元组结构体是一种形式是元组的结构体。
    //与元组的区别是它有名字和固定的类型格式。它存在的意义是为了处理那些需要定义类型（经常使用）又不想太复杂的简单数据：
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);
    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);

    /* 结构体所有权
     * 结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段。
     * 这就是为什么本章的案例中使用了 String 类型而不使用 &str 的原因。
     * 但这不意味着结构体中不定义引用型字段，这需要通过"生命周期"机制来实现。
     */
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1's area is {}", rect1.area());

    let rect = Rectangle::create(30, 50);
    println!("{:?}", rect);
}

//书分为纸质书（Papery book）和电子书（Electronic book）。
#[derive(Debug)]
enum Book {
    Papery,
    Electronic,
}

//如果你现在正在开发一个图书管理系统，你需要描述两种书的不同属性（纸质书有索书号，电子书只有 URL），你可以为枚举类成员添加元组属性描述：
#[derive(Debug)]
enum Book1 {
    Papery(u32),
    Electronic(String),
}

//如果你想为属性命名，可以用结构体语法：
//虽然可以如此命名，但请注意，并不能像访问结构体字段一样访问枚举类绑定的属性。访问的方法在 match 语法中。
#[derive(Debug)]
enum Book2 {
    Papery { index: u32 },
    Electronic { url: String },
}

fn test7_enum() {
    let book = Book::Papery;
    println!("{:?}", book);

    let book = Book1::Papery(1001);
    println!("{:?}", book);
    let ebook = Book1::Electronic(String::from("url://..."));
    println!("{:?}", ebook);

    let book = Book2::Papery { index: 1001 };
    println!("{:?}", book);

    //match语法
    /*
     * 枚举的目的是对某一类事物的分类，分类的目的是为了对不同的情况进行描述。基于这个原理，往往枚举类最终都会被分支结构处理（许多语言中的 switch ）。
     *  switch 语法很经典，但在 Rust 中并不支持，很多语言摒弃 switch 的原因都是因为 switch 容易存在因忘记添加 break 而产生的串接运行问题，
     * Java 和 C# 这类语言通过安全检查杜绝这种情况出现。
     * Rust 通过 match 语句来实现分支结构。
     *
     */

    enum Book3 {
        Papery { index: u32 },
        Electronic { url: String },
    }

    let book = Book3::Papery { index: 1001 };
    let ebook = Book3::Electronic {
        url: String::from("url..."),
    };

    match book {
        Book3::Papery { index } => {
            println!("Papery book {}", index);
        }
        Book3::Electronic { url } => {
            println!("E-book {}", url);
        }
    }

    /*
     * match 块也可以当作函数表达式来对待，它也是可以有返回值的：
     * 但是所有返回值表达式的类型必须一样！如果把枚举类附加属性定义成元组，在 match 块中需要临时指定一个名字：
     */
    enum Book5 {
        Papery(u32),
        Electronic { url: String },
    }
    let book = Book5::Papery(1001);

    match book {
        Book5::Papery(i) => {
            println!("{}", i);
        }
        Book5::Electronic { url } => {
            println!("{}", url);
        }
    }

    /*
     * match 除了能够对枚举类进行分支选择以外，还可以对整数、浮点数、字符和字符串切片引用（&str）类型的数据进行分支选择。
     * 其中，浮点数类型被分支选择虽然合法，但不推荐这样使用，因为精度问题可能会导致分支错误。
     * 对非枚举类进行分支选择时必须注意处理例外情况，即使在例外情况下没有任何要做的事 . 例外情况用下划线 _ 表示：
     */
    let t = "abc";
    match t {
        "abc" => println!("Yes"),
        _ => {}
    }

    //option枚举类
    /*
     * Option 是 Rust 标准库中的枚举类，这个类用于填补 Rust 不支持 null 引用的空白。
     * 许多语言支持 null 的存在（C/C++、Java），这样很方便，但也制造了极大的问题，null 的发明者也承认这一点，"一个方便的想法造成累计 10 亿美元的损失"。
     * null 经常在开发者把一切都当作不是 null 的时候给予程序致命一击：毕竟只要出现一个这样的错误，程序的运行就要彻底终止。
     * 为了解决这个问题，很多语言默认不允许 null，但在语言层面支持 null 的出现（常在类型前面用 ? 符号修饰）。
     * Java 默认支持 null，但可以通过 @NotNull 注解限制出现 null，这是一种应付的办法。
     * Rust 在语言层面彻底不允许空值 null 的存在，但无奈null 可以高效地解决少量的问题，所以 Rust 引入了 Option 枚举类：
     * enum Option<T> {
     *  Some(T),
     *  None,
     * }
     */
    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }

    //如果你的变量刚开始是空值，你体谅一下编译器，它怎么知道值不为空的时候变量是什么类型的呢？
    //所以初始值为空的 Option 必须明确类型：
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }

    //由于 Option 是 Rust 编译器默认引入的，在使用时可以省略 Option:: 直接写 None 或者 Some()。
    //Option 是一种特殊的枚举类，它可以含值分支选择：
    let t = Some(64);
    match t {
            Some(64) => println!("Yes"),
            _ => println!("No"),
    }

    //if let 语法糖
    enum Book6 {
        Papery(u32),
        Electronic(String)
    }
    let book = Book6::Electronic(String::from("url"));
    if let Book6::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }
}
