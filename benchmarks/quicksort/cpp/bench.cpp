#include <iostream>

using namespace std;
extern "C" {
    void start_rapl();
    void stop_rapl();
}

#include <iterator>
#include <algorithm>
#include <functional>
#include <string>
#include <vector>

// helper function for median of three
template<typename T>
 T median(T t1, T t2, T t3)
{
  if (t1 < t2)
  {
    if (t2 < t3)
      return t2;
    else if (t1 < t3)
      return t3;
    else
      return t1;
  }
  else
  {
    if (t1 < t3)
      return t1;
    else if (t2 < t3)
      return t3;
    else
      return t2;
  }
}

// helper object to get <= from <
template<typename Order> struct non_strict_op:
  public std::binary_function<typename Order::second_argument_type,
                              typename Order::first_argument_type,
                              bool>
{
  non_strict_op(Order o): order(o) {}
  bool operator()(typename Order::second_argument_type arg1,
                  typename Order::first_argument_type arg2) const
  {
    return !order(arg2, arg1);
  }
private:
  Order order;
};

template<typename Order> non_strict_op<Order> non_strict(Order o)
{
  return non_strict_op<Order>(o);
}

template<typename RandomAccessIterator,
         typename Order>
 void quicksort(RandomAccessIterator first, RandomAccessIterator last, Order order)
{
  if (first != last && first+1 != last)
  {
    typedef typename std::iterator_traits<RandomAccessIterator>::value_type value_type;
    RandomAccessIterator mid = first + (last - first)/2;
    value_type pivot = median(*first, *mid, *(last-1));
    RandomAccessIterator split1 = std::partition(first, last, std::bind2nd(order, pivot));
    RandomAccessIterator split2 = std::partition(split1, last, std::bind2nd(non_strict(order), pivot));
    quicksort(first, split1, order);
    quicksort(split2, last, order);
  }
}
// test method
template<typename RandomAccessIterator>
 void quicksort(RandomAccessIterator first, RandomAccessIterator last)
{
  quicksort(first, last, std::less<typename std::iterator_traits<RandomAccessIterator>::value_type>());
}

// read a vector of integers from a string (comma seperated)
vector<int> IntVectorFromString(std::string str){
    vector<int> result;

    size_t pos = 0;
    while( pos = str.find(",") != std::string::npos){

        std::string token = str.substr(0, str.find(","));
        result.push_back(std::atoi(token.c_str()));
        str.erase(0, pos + 1);
    }
    result.push_back(std::atoi(str.c_str()));
    return result;
}

int main(int argc, char *argv[]) {

    std::string mergeParamRaw = std::string(argv[2]);

    // removing brackets
    mergeParamRaw.erase(remove(mergeParamRaw.begin(), mergeParamRaw.end(), ']'), mergeParamRaw.end());
    mergeParamRaw.erase(remove(mergeParamRaw.begin(), mergeParamRaw.end(), '['), mergeParamRaw.end());


    // getting numbers from mergeParamRaw
    vector<int> mergeParam = IntVectorFromString(mergeParamRaw);


    int count = std::atoi(argv[1]);

    for (int i = 0; i < count; i++) {
        // copying mergeParam to avoid changing it
        vector<int> mergeParamCopy = vector<int>(mergeParam);

        start_rapl();

        quicksort(mergeParamCopy.begin(), mergeParamCopy.end());

        stop_rapl();

        // stopping compiler optimization
        if (mergeParamCopy.size() < 42){
            std::cout << "Result: " << mergeParamCopy[0] << std::endl;
        }
    }

    return 0;
}