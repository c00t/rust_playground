fn main() {
    //println!("Hello, world!");
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("a:{},b:{}",test1.a(),test1.b());
    println!("a:{},b:{}",test2.a(),test2.b());

    std::mem::swap(&mut test1, &mut test2);
    println!("a:{},b:{}",test1.a(),test1.b());
    println!("a:{},b:{}",test2.a(),test2.b());

    // Use `Pin to stack`
    let mut test1 = TestPinInStack::new("test1");
    let mut test2 = TestPinInStack::new("test2");
    let mut test1 = unsafe {
        Pin::new_unchecked(&mut test1)
    };
    let mut test2 = unsafe {
        Pin::new_unchecked(&mut test2)
    };

    test1.as_mut().init();
    test2.as_mut().init();

    println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
    println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
    //std::mem::swap(test1.get_mut(),test2.get_mut() ); // can't be unpined
}

/// Example of self-referential types
#[derive(Debug)]
struct Test{
    a:String,
    b:*const String,
}
impl Test{
    fn new(s:&str)->Self{
        Test{
            a:String::from(s),
            b:std::ptr::null(),
        }
    }
    fn init(&mut self){
        self.b = &self.a;
    }
    fn a(&self)->&str{
        &self.a
    }
    fn b(&self)->&str{
        assert!(!self.b.is_null(),"Not Init!!");
        unsafe{&*self.b}
    }
}

/// Example Pin to stack
use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct TestPinInStack{
    a:String,
    b:*const String,
    _marker:PhantomPinned,
}
impl TestPinInStack{
    fn new(s:&str)->Self{
        TestPinInStack{
            a:String::from(s),
            b:std::ptr::null(),
            _marker:PhantomPinned,
        }
    }
    // 没有返回东西啊，没必要加'a标识吧？
    fn init(self:Pin<&mut Self>){
        // 这里如果不强制指明`*const String`类型，这一条会被Borrow checker阻止
        let self_ref:*const String = &self.a;
        let this = unsafe {
            self.get_unchecked_mut()
        };
        this.b = self_ref;
    }
    // 这里应该会自动推断啊？没必要加'a吧？
    fn a(self:Pin<& Self>)->&str{
        &self.get_ref().a
    }
    // 这里应该也会自动推断啊？也没必要加'a吧？
    fn b(self:Pin<&Self>)->&String{
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe{
            &(*self.b)
        }
    }
}