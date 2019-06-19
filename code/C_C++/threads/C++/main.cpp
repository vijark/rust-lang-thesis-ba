#include <iostream>
#include <mutex>
#include <thread>

using namespace std;

int count;
mutex m;

void do_something() {
    // On construction, the mutex object is locked by the calling thread, and on
    // destruction, the mutex is unlocked.
    lock_guard<mutex> guard(m);

    cout << "job " << ++count << " started" << endl;
    this_thread::sleep_for(1s);
    cout << "job " << count << " finished" << endl;
}

int main() {
    thread t1(do_something);
    thread t2(do_something);

    t1.join();
    t2.join();

    cout << "thread stopped" << endl;

    return 0;
}
