in rust, it is common to compose functions together,
however in order to compose function together, with have to deal 
with lots of nested of parathensis, you commonly use the method 
syntax. 

this creates traits that apply to every type, 
that allow "piping" data through, a free function or closure
in a method chain without breaking the format of method chain.

in addition, this code adds pipmoder trait, that additionally,
allows chaining functions, that return unit but modify self,
by returning self
