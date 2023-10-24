#include <iostream>

extern "C" {
    void start_rapl();
    void stop_rapl();
}

#include <iterator>
#include <algorithm> // for std::inplace_merge
#include <functional> // for std::less
#include <string>
#include <vector>

template<typename RandomAccessIterator, typename Order>
 void mergesort(RandomAccessIterator first, RandomAccessIterator last, Order order)
{
  if (last - first > 1)
  {
    RandomAccessIterator middle = first + (last - first) / 2;
    mergesort(first, middle, order);
    mergesort(middle, last, order);
    std::inplace_merge(first, middle, last, order);
  }
}

template<typename RandomAccessIterator>
 void mergesort(RandomAccessIterator first, RandomAccessIterator last)
{
  mergesort(first, last, std::less<typename std::iterator_traits<RandomAccessIterator>::value_type>());
}


using namespace std;

int main(int argc, char *argv[]) {

    std::string mergeParamRaw = std::string(argv[1]);

    // removing brackets
    mergeParamRaw.erase(remove(mergeParamRaw.begin(), mergeParamRaw.end(), ']'), mergeParamRaw.end());
    mergeParamRaw.erase(remove(mergeParamRaw.begin(), mergeParamRaw.end(), '['), mergeParamRaw.end());
    

    // getting numbers from mergeParamRaw
    vector<int> mergeParam(1, 0);
 
    int j = 0;
    for (int i = 0; i < mergeParamRaw.size(); i++) {
        // s[i] - '0' would also work here
        mergeParam[j] = mergeParam[j] * 10 + (mergeParamRaw[i] - 48);
    }

    int count = std::atoi(argv[2]);

    for (int i = 0; i < mergeParam.size(); i++) {
        std::cout << mergeParam[i] << " ";
    }

    for (int i = 0; i < count; i++) {
        // copying mergeParam to avoid changing it
        vector<int> mergeParamCopy = vector<int>(mergeParam);

        start_rapl();

        mergesort(mergeParamCopy.begin(), mergeParamCopy.end());

        stop_rapl();

        for (int i = 0; i < mergeParamCopy.size(); i++) {
            std::cout << mergeParamCopy[i] << " ";
        }

        // stopping compiler optimization
        if (mergeParamCopy.size() < 42){
            std::cout << "Result: " << mergeParamCopy[0] << std::endl;
        }
    }

    return 0;
}
