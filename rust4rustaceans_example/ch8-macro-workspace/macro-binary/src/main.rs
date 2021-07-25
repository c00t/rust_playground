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

    /// Test Rust macro's hygiene
    macro_rules! let_make_a_foo {
        ($x:expr) => {
            let foo_test = $x;
        }
    }
    let foo_test = 1; // (1)
    let_make_a_foo!(2);// 展开为 let foo = 2;
    assert_eq!(foo_test,1);// 并没有sahdow (1)中的foo_test
    // 想要影响到caller's scope的话，可以使用以下方法
    macro_rules! let_make_a_shadow_foo{
        ($x:ident,$y:expr) => {
            let $x = $x + 1;
        }
    }
    let foo_test = 1; // (1)
    let_make_a_shadow_foo!(foo_test,foo_test + 1);// 这里显式引入了foo_test，和之前强调的点是一样的，需要注意的是`foo_test+1`同样也引入了`foo_test`。
    assert_eq!(foo_test,2);

    /// Test TRR Proc-Macro examples in binary
    use proc_macro_examples::make_answer;
    make_answer!();
    println!("{}",answer());

    /// Test TRR Derive-Proc-Macro examples in binary
    use proc_macro_examples::AnswerFnDerive;
    #[derive(AnswerFnDerive)]
    struct SSS;
    // `derive(...)` add an answer_derive()->u32{42} here
    let ss = SSS{};//这里的SSS的定义并不会消失。
    println!("{}",answer_derive());

    /// Test TRR derive-macro-helper-attributes example in binary
    use proc_macro_examples::ZZZZ;
    #[derive(ZZZZ)]
    struct SSSS{
        #[helper1] field1:i32,
        #[helper2] field2:i32,
        //#[helper3] field3:i32, error here, not defined.
    };

    /// Test TRR attribute macros example in binary
    use proc_macro_examples::show_streams;
    #[show_streams(barbar)]
    fn invoke1(){}


    /// Test R4R token span example
    macro_rules! name_as_debug{
        ($t:ty)=>{
            impl ::core::fmt::Debug for $t {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result{
                    ::core::write!(f,::core::stringify!($t))
                }
            }
        }
    }
    name_as_debug!(u31);
}
