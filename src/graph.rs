/*!

# Graph Oriented Modelling

根据GraphQL的规范，定义了Query, Mutation 和 Subscription 三个executing operations。基于此，Graph trait也实现上述三个methods：query, mutate, subscribe。

Graph 的核心是定位vertices并对其进行操作，然后返回结果。输入项被称为谓语：Predicate，输出项：Output。根据Rust的设计理念，Predicate作为输入项，采用generic parameter比较好，Output作为输出项，采用associated type为好。举一个例子，如果对于非对象类型的vertex，可以统一使用usize进行索引访问（视同数组），这时可以单独对usize类型的predicate作为特例进行实现。

rust语言 immutability/mutability 设计与GraphQL吻合。从安全性考虑的访问权限限定以及从效率考虑的独占和共享访问分离，都需要把Graph的行为进行分类。独立设计只读操作的Query和读写操作的Mutation。Mutation继承Query，而Graph继承Mutation并自带subscription行为。

Graph中的两个基本元素是vertex和edge。不考虑过多优化的前提下，可以对graph, vertex和edge进行归一化设计。亦即面向vertex，实现graph行为，嵌入各自的edge。通过vertex的自组织形成相对松散的graph结构。未来进行优化时，再考虑实现一个统一的graph数据结构层。

极端扁平化设计时，每个vertex只能是原子化值，不可再分割。实际应用中，为了提升效率，vertex通常是一个对象容器，内部包含一个key-value树，也就是一个单向无环图，因此vertex可以被视为graph。使用RDF N-Quad进行mutation操作时，需要知晓每条操作所在的graph 名称，设定为内部graph时，是对vertex内部tree的操作。

Graph中以任何一个vertex作为根结点，可以实现路径寻址，而这个路径是由edges连接。Graph实现可以很简单，也可以很复杂。当前的crate.io库中还没有的graph项目适合直接拿来用，不过有些抽象graph算法库可以做到backend无关。

引入Graph概念最主要的出发点是降低系统的复杂性，提升灵活性以及伸缩性。从降低系统复杂性的角度，在设计中我们引入`Autism`设计理念，每个vertex只需要关注自身，无需了解整个系统。每个vertex自成一体，由时空两个维度的两个Graph组成，一个是vertex内部数据组成的树状单向无环图，另一个是作为有限状态机的状态图。其中空间图统一化保存内部数据和外部链接。表面看，内部数据与外部链接没有什么不同。只有把其中保存的数据当作另一个vertex的索引时，它才呈现为外部edge的特征。设计时可以只专注于该vertex本身，以及必要时通过一个全局的Registrar索引到自己的直接邻居进行交互。为了简化设计，当前的版本中暂时没有加入FSM状态图的支持，未来将会有一个数据结构用于存储该vertex的状态图。也不排除未来归一化设计，把时间维度的状态流转合并到空间关系中，也即不同状态下的vertex会被视为不同的vertex。vertex通过实现State接口支持当下的单一状态的状态机模式，未来加入对状态集合及状态图的支持。

Graph是理论化模型，为了更加贴近实际应用，我们会在后续的系统模块中引入事务（Thing）的概念。Thing是Graph/Vertex/Edge的综合体，详细描述见`ns-something`模块。

ns当前的思路是：

1. 借鉴GraphQL设计思想，并考虑未来全面支持GraphQL。
2. 化整为零，每个vertex都是以其为视角的graph，它只与有路径直接可达的邻居进行交互。
3. vertex沉浸在一个Global环境中，可以通过uuid访问到其他vertex。一个vertex一旦知晓另一个vertex的uuid，则视作二者建立了直接联系。
4. vertex支持Index，实现类似数组及HashMap的`[key]`级联操作，用于路径遍历。
5. 不再设计Graph及Edge数据结构，直接使用vertex的索引机制实现。每个vertex负责自己的edge。
6. ns有一个系统的根vertex，System。这个System挂接一级节点，并支持预加载以bootstrap整个系统。System中维护一个以uuid为索引的扁平HashMap，用于uuid快速定位所有vertex。
7. vertex内部的edge容器是一个以Value为key的二叉树，叶节点可以是uuid、Thing, `Arc<RwLock<Thing>>`或者`Rc<Ref<Thing>>`，分别应对网络分布、本vertex内部、进程和线程使用环境。
8. graph行为需要动态进行挂载，理想方式是对 Trait vtable 进行hack，把外部独立函数挂载到相应的trait上。考虑的unsafe及其他原因，先使用相对低效的函数指针。这种模式下，只能把Thing内部数据完全开放出去。

面向Vertex设计模式下，可以只见树木不见森林，只需要专注于当前vertex。vertex的核心操作是graph的addressing，可以把内部和外部邻居均视作当前vertex的edge。对于内部数据寻址，键值是泛型Value，返回值也是Value；而对于邻居，索引方式相同，只是再增加一步在全局Hash表中获取邻居后再进行后续交互操作。

基于以前的开发经验，ns_base提供紧耦合和松耦合两种交互接口。其中Graph接口是为了高效的数据访问，通常采用紧耦合接口；Actor为了分布式计算，采用松耦合接口。通常设计中内部访问采用Graph接口，外部访问采用Actor接口；外部访问时可以使用Actor接口代理Graph接口操作。

为了方便大量的数据访问，也提供了基于CRUD的Access接口，用于内部快速数据存取。作为语法糖，还默认实现了Index/IndexMut接口，用于更为高效的数据引用操作。

# 代码

```rust
//
*/

pub trait Scalar<T> {
    /// Get reference of inner T
    fn scalar(&self) -> Option<&T> {
        None
    }

    /// Check is if type T
    fn is_scalar(&self) -> bool {
        self.scalar().is_some()
    }

    /// Clone inner T then output
    fn to_scalar(&self) -> Option<T> {
        None
    }

    /// Convert inner data to type T.
    fn as_scalar(&self) -> Option<T> {
        None
    }
}

pub trait Hyperedge<T: ?Sized> {
    type Output: ?Sized;

    fn get(self, vector: &T) -> Option<&Self::Output>;
    fn get_mut(self, vector: &mut T) -> Option<&mut Self::Output>;
}

pub trait Vector<I>
where
    I: Hyperedge<Self>,
{
    type Output;

    fn size(&self) -> usize;

    fn get(&mut self, _: I) -> Self::Output;

    fn put(&mut self, index: I, _: I::Output) -> Self::Output;

    fn delete(&mut self, _: I) -> Self::Output;

    fn new(&mut self, _: I::Output) -> Option<I>;

    fn get_ref(&self, index: I) -> Option<&I::Output> {
        index.get(self)
    }

    fn get_mut(&mut self, index: I) -> Option<&mut I::Output> {
        index.get_mut(self)
    }

    fn with<F: FnOnce(&I::Output)>(&self, index: I, f: F) -> Self::Output;

    fn with_mut<F: FnOnce(&mut I::Output)>(&mut self, index: I, f: F) -> Self::Output;
}

/// 通用的Graph行为，参考了GraphQL的规格。
pub trait Graph<P> {
    type Result;

    /// 字面上是查询调用，不改变Graph状态。由于Rust有Interior Mutability, 是有可能改变内部状态的。
    fn query(&self, _predicate: P) -> Self::Result {
        unimplemented!()
    }

    /// 在GraphQL中，主要用于状态修改类的操作。在Rust体系中，这个操作更多是独占和排他的意味。
    fn mutate(&mut self, _predicate: P) -> Self::Result {
        unimplemented!()
    }

    /// 订阅Graph事件，作为回调函数，被Graph反向调用，用于数据向外主动推送。
    fn subscribe(&mut self, _predicate: P) -> Self::Result {
        unimplemented!()
    }
}

/// Actor model 的抽象。通常Graph和Access是紧耦合接口，Actor则是松耦合接口。Actor唯一的函数类似于消息的传递。
pub trait Actor<M> {
    type Response;

    /// 通常用作松耦合的消息传递函数，效率低点，普适性好些。相对于其他函数调用，这个更多用于异步单向的消息传递。与 Directed Graph 中的消息传递很搭。
    fn invoke(&self, _predicate: M) -> Self::Response {
        unimplemented!()
    }
}

/// FSM model的抽象。实际是FSM和State一体的。Actor可以认为是单一状态的FSM。在大多数单一状态FSM/Actor中，被当作构造函数和析构函数。
/// 作为状态函数，State有输入/输出和状态迁移的entry/exit。
/// 当entry输出的是一个stream序列时，内部是状态机在运转。
pub trait State {
    type Input;
    type Output;
    fn entry(self, _arg: Self::Input) -> Self::Output;
    fn exit(&mut self) {}
}

/*
```
*/
