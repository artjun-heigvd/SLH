#include <cstddef>
#include <iostream>

std::string reverse(std::string reverse){

    std::string res = "";

    for(size_t i = reverse.length(); i > 0; --i){
        res = res + reverse.at(i - 1);
    }

    return res;
}


int main() {
    std::string toReverse = "Hello world!";
    
    std::cout << reverse(toReverse) << std::endl;
}