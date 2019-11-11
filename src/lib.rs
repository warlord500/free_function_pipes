/// the point of this trait, is allow you to use, free functions, in composing manner 
///without being difficult to read with lots of nested parethensis


trait FreeFunPiper : Sized {
   #[inline(always)] 
     fn pipe<U,Fun>(self, f: Fun) -> U 
		where 
		 Fun : FnOnce(Self) -> U,
         U : Sized
    {
        f(self)
    }

   #[inline(always)] 
     fn pipe2<U,A,Fun>(self, f: Fun, a : A) -> U 
		where 
		 Fun : FnOnce(Self,A) -> U,
         U : Sized
    {
        f(self,a)
    }

   #[inline(always)] 
     fn pipe3<A,B,U,Fun>(self, f : Fun,
                               a : A,
                               b : B) -> U 
		where 
		 Fun : FnOnce(Self,A,B) -> U 
    {
        f(self,a,b)
    }

   #[inline(always)] 
     fn pipe4<A,B,C,U,Fun>(self, f : Fun,
                                 a : A,
                                 b : B,
                                 c : C) -> U 
		where 
		 Fun : FnOnce(Self,A,B,C) -> U 
    {
        f(self,a,b,c)
    }

   #[inline(always)] 
     fn pipe5<A,B,C,D,U,Fun>(self, f : Fun,
                                   a : A,
                                   b : B, 
                                   c : C,
                                   d : D) -> U 
		where 
		 Fun : FnOnce(Self,A,B,C,D) -> U 
    {
        f(self,a,b,c,d)
    }
///truthfully if you need more data than this 
///you have a real problem, you should probably merge some
///arguments under a structure
   #[inline(always)] 
     fn pipe6<A,B,C,D,E,U,Fun>(self, f : Fun,
                                     a : A, 
                                     b : B, 
                                     c : C, 
                                     d : D,
                                     e : E) -> U 
		where 
		 Fun : FnOnce(Self,A,B,C,D,E) -> U 
    {
        f(self,a,b,c,d,e)
    } 
}
//add piper trait to nearly every type that it can 
impl<T> FreeFunPiper for T where T : Sized {}

///this trait allows you to use functions, 
///that might normally might not be chainable, 
///using pipes from above trait!
///in theory using this should be no slower than 
///pipe because, move ellision should eliminate most of the moves.
///making pipm chaining just as fast explicit pipe writers 
trait FreeFunPipModer : Sized {

   #[inline(always)] 
     fn pipm<Fun>(mut self, f: Fun) -> Self 
		where 
		 Fun : FnOnce(&mut Self) 
    {
        f(&mut self);
            self
    }

   #[inline(always)] 
     fn pipm2<A,Fun>(mut self, f: Fun, a : A) -> Self 
		where 
		 Fun : FnOnce(&mut Self,A) 
    {
        f(&mut self,a);
            self
    }

   #[inline(always)] 
     fn pipm3<A,B,Fun>(mut self, f : Fun,
                               a : A,
                               b : B) -> Self 
		where 
		 Fun : FnOnce(&mut Self,A,B) 
    {
        f(&mut self,a,b);
            self
    }

   #[inline(always)] 
     fn pipm4<A,B,C,Fun>(mut self, f : Fun,
                                 a : A,
                                 b : B,
                                 c : C) -> Self 
		where 
		 Fun : FnOnce(&mut Self,A,B,C) 
    {
        f(&mut self,a,b,c);
            self
    }

   #[inline(always)] 
     fn pipm5<A,B,C,D,Fun>(mut self, f : Fun,
                                   a : A,
                                   b : B, 
                                   c : C,
                                   d : D) -> Self 
		where 
		 Fun : FnOnce(&mut Self,A,B,C,D) 
    {
        f(&mut self,a,b,c,d);
            self
    }
///truthfully if you need more data than this 
///you have a real problem, you should probably merge some
///arguments under a structure
   #[inline(always)] 
     fn pipm6<A,B,C,D,E,Fun>(mut self, f : Fun,
                                     a : A, 
                                     b : B, 
                                     c : C, 
                                     d : D,
                                     e : E) -> Self 
		where 
		 Fun : FnOnce(&mut Self,A,B,C,D,E) 
    {
        f(&mut self,a,b,c,d,e);
        self

    } 
}
///blanket implement, because is really only limited,
///by the fact that T
impl<T> FreeFunPipModer for T where T : Sized {}


