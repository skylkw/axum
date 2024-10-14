use chrono::{DateTime, Utc};
use itertools::Itertools;
use std::{collections::HashMap, hash::Hash};

use crate::dto::request::Direction;

/// 比较两个切片是否包含相同的元素（包括重复元素的数量）。
///
/// #### 参数
///
/// - `result`: 第一个切片。
/// - `expected`: 第二个切片。
///
/// #### 返回值
///
/// 如果两个切片包含相同的元素，则返回 `true`，否则返回 `false`。
///
/// #### 示例
///
/// ```
/// let a = vec![1, 2, 2, 3];
/// let b = vec![3, 2, 1, 2];
/// assert!(eq(&a, &b));
/// ```
pub fn eq<T>(result: &[T], expected: &[T]) -> bool
where
    T: Eq + Hash,
{
    fn count<T>(items: &[T]) -> HashMap<&T, usize>
    where
        T: Eq + Hash,
    {
        let mut cnt = HashMap::new();
        for i in items {
            *cnt.entry(i).or_insert(0) += 1
        }
        cnt
    }
    count(result) == count(expected)
}

/// 比较两个向量是否相等。
///
/// #### 参数
///
/// - `a`: 第一个向量。
/// - `b`: 第二个向量。
///
/// #### 返回值
///
/// 如果两个向量相等，则返回 `true`，否则返回 `false`。
///
/// #### 示例
///
/// ```
/// let a = vec![1, 2, 3];
/// let b = vec![1, 2, 3];
/// assert!(vecs_match(&a, &b));
/// ```
pub fn vecs_match<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.len() == b.len() && !a.iter().zip(b.iter()).any(|(a, b)| *a != *b)
}

/// 比较两个日期时间是否相等（精确到分钟）。
///
/// #### 参数
///
/// - `left`: 第一个日期时间。
/// - `right`: 第二个日期时间。
///
/// #### 返回值
///
/// 如果两个日期时间相等，则返回 `true`，否则返回 `false`。
///
/// #### 示例
///
/// ```
/// use chrono::Utc;
/// let dt1 = Utc::now();
/// let dt2 = dt1.clone();
/// assert!(compare_datetime(&dt1, &dt2));
/// ```
pub fn compare_datetime(left: &DateTime<Utc>, right: &DateTime<Utc>) -> bool {
    left.format("%d/%m/%Y %H:%M").to_string() == right.format("%d/%m/%Y %H:%M").to_string()
}

/// 检查一个元素是否存在于一个切片中。
///
/// #### 参数
///
/// - `haystack`: 要搜索的切片。
/// - `needle`: 要查找的元素。
///
/// #### 返回值
///
/// 如果元素存在于切片中，则返回 `true`，否则返回 `false`。
///
/// #### 示例
///
/// ```
/// let haystack = vec![1, 2, 3];
/// let needle = 2;
/// assert!(exist(&haystack, &needle));
/// ```
pub fn exist<T>(haystack: &[T], needle: &T) -> bool
where
    T: PartialEq,
{
    haystack.iter().any(|i| i == needle)
}

/// 检查一个切片中的所有元素是否都存在于另一个切片中。
///
/// #### 参数
///
/// - `haystack`: 要搜索的切片。
/// - `handful`: 要查找的元素切片。
///
/// #### 返回值
///
/// 如果所有元素都存在于切片中，则返回 `true`，否则返回 `false`。
///
/// #### 示例
///
/// ```
/// let haystack = vec![1, 2, 3, 4, 5, 6];
/// let handful = vec![1, 2, 6];
/// assert!(exist_all(&haystack, &handful));
/// ```
pub fn exist_all<T>(haystack: &[T], handful: &[T]) -> bool
where
    T: PartialEq,
{
    handful.iter().all(|i| haystack.contains(i))
}

/// 检查一个集合是否按指定方向排序。
///
/// #### 参数
///
/// - `items`: 要检查的集合。
/// - `direction`: 排序方向。
///
/// #### 返回值
///
/// 如果集合按指定方向排序，则返回 `true`，否则返回 `false`。
///
/// #### 示例
///
/// ```
/// use crate::dto::request::Direction;
/// let items = vec![1, 2, 3];
/// assert!(is_sorted(items, Direction::ASC));
/// ```
pub fn is_sorted<I>(items: I, direction: Direction) -> bool
where
    I: IntoIterator,
    I::Item: Ord + Clone,
{
    items
        .into_iter()
        .tuple_windows()
        .all(direction.as_closure())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exist_assertion() {
        let h = vec![1, 2, 3];
        let n = 2;
        assert!(exist(&h, &n))
    }

    #[test]
    fn test_not_exist_assertion() {
        let h = vec![1, 2, 3];
        let n = 20;
        assert!(!exist(&h, &n))
    }

    #[test]
    fn exist_all_test() {
        let h = vec![1, 2, 3, 4, 5, 6];
        let n = vec![1, 2, 6];
        assert!(exist_all(&h, &n))
    }

    #[test]
    fn test_not_exist_all_assertion() {
        let h = vec![1, 2, 3];
        let n = vec![1, 2, 60];
        assert!(!exist_all(&h, &n))
    }

    #[test]
    fn test_is_sort_assertion() {
        let a = vec![1, 20, 3];
        let b = vec![1, 2, 60];
        let c = vec![100, 20, 6];
        let d = vec![100, 20, 60];
        assert!(!is_sorted(a, Direction::ASC));
        assert!(is_sorted(b, Direction::ASC));
        assert!(is_sorted(c, Direction::DESC));
        assert!(!is_sorted(d, Direction::DESC))
    }
}