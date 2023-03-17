pub fn learn_marco() {
    learn_marco_rules();
    if false {
        //
    }
}

macro_rules! say_hello {
    ($e:expr) => {
        $e + 3
    };
}

macro_rules! repeat_catch {
    (
        $(
        $e: expr
        )
        ,
        *
    ) => {
        {
            let mut v = Vec::new();
            $(
            v.push(format!("{}", $e));
        )*
            v
        }
    };
}

// 编译过不去
fn learn_marco_rules() {
    let fib = {
        struct Recurrence {
            mem: [u64; 2],
            pos: usize,
        }
        impl Iterator for Recurrence {
            type Item = u64;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if self.pos < 2 {
                    let next_val = self.mem[self.pos];
                    self.pos +=1;
                    Some(next_val);
                } else {
                   let a = ...;
                }
            }
        }
    };

    println!("{}", say_hello![1]);
    println!("{:?}", repeat_catch![1, "a", true, 3.24159f32]);
    //#[marco_export]
    //marco_rules! vec {
    //    ( $( $x:expr),* ) => {
    //        {
    //            let mut temp_vec = Vec::new();
    //            $(
    //            temp_vec.push($x);
    //            )*
    //            temp_vec
    //        }
    //    };
    //}
}
