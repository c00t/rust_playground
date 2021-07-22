use std::cell::RefCell;

fn main() {
    //println!("Hello, world!");

    // used as an expression
    let x = vec![1,2,3];

    // used as a statement
    println!("hello world");

    // used in a pattern
    // 其实这语法感觉有点像前端了
    macro_rules! pat {
        ($i:ident) => (Some($i))
    }

    if let pat!(x) = Some(1) {
        assert_eq!(x,1);
    }

    // used in a type.
    macro_rules! Tuple {
        {$A:ty,$B:ty} => {($A,$B)};//这一步为什么要加上`;`号呢？测试出来了，其实没有也不影响的。
    }

    type N2 = Tuple!(i32,i32);

    let y : N2 = (0,0);

    // used as an item
    thread_local! {
        static FOO: RefCell<u32> = RefCell::new(1);
    }

    // used as an associated item.
    macro_rules! const_maker {
        ($t:ty,$v:tt) => { const CONST:$t = $v;};
    }
    trait T {
        const_maker!{i32,1} //使用花括号调用也可以
    }

    // 在macros中调用macro
    macro_rules! example {
        () => { println!("Macro called in a macro!") };
    }

    example!();

    // create an local ambiguity error
    // macro_rules! ambiguity {
    //     ($($i:ident)* $j:ident) => {};
    // }
    // let x = 1;
    // let y = 1;
    // ambiguity!(x);

    // 上面的例子，使用下面这种表示就对了
    macro_rules! ambiguity {
        ($($i:ident)+ $($j:ident)?) => {};
    }
    let x = 1;
    let y = 1;
    ambiguity!(x);

    // test call macro by ident&expr
    // use iterally token is false
    // macro_rules! foo {
    //     ($l:expr) => {bar!($l);}
    // }
    // macro_rules! bar {
    //     (3) => {}
    // }
    // foo!(3);
    // use `tt` is correct
    macro_rules! foo {
        ($l:tt) => {bar!($l);}
    }
    macro_rules! bar {
        (3) => {}
    }
    foo!(3);

    // test macro repetition
    macro_rules! repeat_divide_macro {
        ($($i:tt),+) => {
            $($i)/+//连续divide可以了，但是，如果我想要进行连续+的话要怎么弄呢？#spark# 在目前的情况下，这是不可能的。
        }
    }
    let x = repeat_divide_macro!(4,2,1);
    println!("repeat divide result:{}",x);

    // test multi-define shadow
    macro_rules! mm {
        () => {
            println!("1");
        };
    }
    macro_rules! mm {
        () => {
            println!("2");
        };
    }
    mm!();
}
