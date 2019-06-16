#include <iostream>
#include <list>
#include <string>

using namespace std;

class Employee {
  public:
    string first_name, family_name;
    virtual string full_name() { return first_name + " " + family_name; };
    Employee(string, string);
    Employee();
};

Employee::Employee(string first, string family) {
    first_name = first;
    family_name = family;
}

Employee::Employee() {
    first_name = "";
    family_name = "";
}

class Manager : public Employee {
  public:
    list<Employee *> group;
    short level;
    Manager(string, string, list<Employee *>, short);
    string full_name() override;
};

Manager::Manager(string first, string family, list<Employee *> l, short lvl) {
    first_name = first;
    family_name = family;
    group = l;
    level = lvl;
}

string Manager::full_name() {
    return first_name + " " + family_name + " (level " + to_string(level) + ")";
}

void foo(Employee e) {
    cout << "foo: " << e.family_name << ", " << e.first_name << endl;
}

void foo2(Employee e1, Employee e2) {
    cout << "foo2: ";
    foo(e1);
    cout << "foo2: ";
    foo(e2);
}

int main() {
    Employee e("Hans", "Hintermeyer");

    Employee tmp("a", "b");
    list<Employee *> emp{&tmp};
    Manager m("Andy", "Arbeit", emp, 1);

    foo(*m.group.front());
    foo(m);
    foo(e);

    foo2(*m.group.front(), m);

    cout << "Name is <" << e.full_name() << ">" << endl;
    cout << "Name is <" << m.full_name() << ">" << endl;

    return 0;
}
