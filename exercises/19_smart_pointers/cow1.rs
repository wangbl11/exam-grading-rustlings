// This exercise explores the `Cow` (Clone-On-Write) smart pointer. It can
// enclose and provide immutable access to borrowed data and clone the data
// lazily when mutation or ownership is required. The type is designed to work
// with general borrowed data via the `Borrow` trait.

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            /*
            input.to_mut() 方法会检查 input 的类型：
            如果 input 是 Cow::Borrowed(&[i32])，则会克隆数据并将其转换为 Cow::Owned(Vec<i32>)。
            如果 input 已经是 Cow::Owned(Vec<i32>)，则直接返回可变引用。
             */
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // Clone occurs because `input` needs to be mutated.
        /*
        初始状态: input 是 Cow::Borrowed(&vec)，引用 [-1, 0, 1]。
            处理过程:
            第一个元素 -1 小于 0，调用 input.to_mut() 克隆数据并转换为 Cow::Owned(Vec<i32>)，然后将 -1 修改为 1。
            第二个元素 0 不小于 0，不进行修改。
            第三个元素 1 不小于 0，不进行修改。
            最终状态: input 是 Cow::Owned(vec![1, 0, 1])。
         */
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // No clone occurs because `input` doesn't need to be mutated.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        /*
        初始状态: input 是 Cow::Borrowed(&vec)，引用 [0, 1, 2]。
        处理过程:
        所有元素都不小于 0，不进行任何修改。
        最终状态: input 仍然是 Cow::Borrowed(&vec)，没有进行克隆。
         */
        assert!(matches!(input, Cow::Borrowed(_)));
    }

    #[test]
    fn owned_no_mutation() {
        // We can also pass `vec` without `&` so `Cow` owns it directly. In this
        // case, no mutation occurs (all numbers are already absolute) and thus
        // also no clone. But the result is still owned because it was never
        // borrowed or mutated.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        /*
        初始状态: input 是 Cow::Owned(vec![0, 1, 2])，拥有数据的所有权。
        处理过程:
        所有元素都不小于 0，不进行任何修改。
        最终状态: input 仍然是 Cow::Owned(vec![0, 1, 2])，没有进行克隆。
         */
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn owned_mutation() {
        // Of course this is also the case if a mutation does occur (not all
        // numbers are absolute). In this case, the call to `to_mut()` in the
        // `abs_all` function returns a reference to the same data as before.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        /*
        初始状态: input 是 Cow::Owned(vec![-1, 0, 1])，拥有数据的所有权。
        处理过程:
        第一个元素 -1 小于 0，调用 input.to_mut() 直接修改为 1。
        第二个元素 0 不小于 0，不进行修改。
        第三个元素 1 不小于 0，不进行修改。
        最终状态: input 仍然是 Cow::Owned(vec![1, 0, 1])，没有进行额外的克隆。
         */
        assert!(matches!(input, Cow::Owned(_)));
    }
}
