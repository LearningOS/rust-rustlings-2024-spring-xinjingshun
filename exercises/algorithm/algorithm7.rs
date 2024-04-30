/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

// I AM DONE
// 阶梯思路：使用栈实现括号匹配的函数 bracket_match。遍历输入字符串中的每个字符，当遇到左括号时，将其压入栈中；当遇到右括号时，与栈顶的左括号进行匹配，如果匹配成功弹出栈顶的左括号，继续遍历；如果匹配失败或者栈为空，则返回 false。

#[derive(Debug)]
struct Stack<T> {
	size: usize,
	data: Vec<T>,
}
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {
		0 == self.size
	}
	fn len(&self) -> usize {
		self.size
	}
	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}
	fn pop(&mut self) -> Option<T> {
		if self.size > 0 {
			self.size -= 1; // 减少栈的大小
			self.data.pop() // 弹出栈顶元素并返回
		} else {
			None // 如果栈为空，返回 None
		}
	}

	fn peek(&self) -> Option<&T> {
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn bracket_match(bracket: &str) -> bool {
	let mut stack = Stack::new(); // 创建一个栈用于存储左括号

	// 遍历输入字符串中的每个字符
	for c in bracket.chars() {
		match c {
			'(' | '[' | '{' => {
				stack.push(c); // 遇到左括号，压入栈中
			}
			')' => {
				// 遇到右括号')'，尝试匹配栈顶的左括号'('
				if let Some('(') = stack.pop() {
					continue; // 匹配成功，继续遍历
				} else {
					return false; // 匹配失败，返回 false
				}
			}
			']' => {
				// 遇到右括号']'，尝试匹配栈顶的左括号'['
				if let Some('[') = stack.pop() {
					continue; // 匹配成功，继续遍历
				} else {
					return false; // 匹配失败，返回 false
				}
			}
			'}' => {
				// 遇到右括号'}'，尝试匹配栈顶的左括号'{'
				if let Some('{') = stack.pop() {
					continue; // 匹配成功，继续遍历
				} else {
					return false; // 匹配失败，返回 false
				}
			}
			_ => {
				// 忽略其他字符，继续遍历
				continue;
			}
		}
	}

	// 遍历结束后，如果栈为空，则括号匹配成功，否则匹配失败
	stack.is_empty()
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}