#include <stdlib.h>
#include <stdio.h>
#include <string.h>

struct dealer_t {
    char *street_name;
    int cocaine_grams;
    int money_euros;
    int selling_price;
};
typedef struct dealer_t *DEALER;

struct customer_t {
    char *name;
    int money_euros;
    int fidelity_points;
};
typedef struct customer_t *CUSTOMER;

char input_buffer[64];


//Mocked SQL Database - assume no vulnerability

CUSTOMER get_customer(char *username, char *password) {
    // SELECT * FROM customer WHERE username = $username and password = $password
    CUSTOMER ptr = (CUSTOMER)malloc(sizeof (struct customer_t));
    ptr->name = strdup(username);
    ptr->money_euros = 300;
    ptr->fidelity_points = 0;
    return ptr;
}

DEALER get_dealer(char *street_name) {
    // SELECT * FROM dealer WHERE street_name = $street_name
    DEALER ptr = (DEALER)malloc(sizeof (struct dealer_t));
    ptr->street_name = strdup(street_name);
    ptr->cocaine_grams = 100;
    ptr->money_euros = 0;
    ptr->selling_price = 70;
    return ptr;
}

void save_dealer(DEALER dealer) {
    return;
}

void save_customer(CUSTOMER customer) {
    return;
}

// End of mocked SQL database

void dispatch_order(char *customer, char *street_name, int amount) {
    printf("Dispatching delivery of %d grams from %s to %s.\n", amount, street_name, customer);
}


int transaction(char *dealer_name, char *user, char *password, int grams) {

    CUSTOMER customer = get_customer(user, password);
    printf("Welcome, %s.\n", customer->name);

    DEALER dealer = get_dealer(dealer_name);
    int cost = grams * dealer->selling_price;

    printf("You are buying %d grams of cocaine for %d euros\n", grams, cost);

    if (grams > dealer->cocaine_grams) {
        printf("Sorry, not enough drugs available.\n");
        return 1;
    }

    if (cost > customer->money_euros) {
        printf("Sorry, you cannot afford that much.\n");
        return 2;
    }

    dispatch_order(customer->name, dealer->street_name, grams);
    customer->money_euros -= cost;
    dealer->money_euros += cost;
    dealer->cocaine_grams -= grams;

    customer->fidelity_points += 1;

    save_customer(customer);
    save_dealer(dealer);

    free(customer);
    free(dealer);

    return 0;

}

int main(int argc, char **argv) {
    if (argc != 5) {
        fprintf(stderr, "Usage: market <dealer> <user> <password> <amount>\n");
        return 1;
    }
    char *dealer_name = argv[1];
    char *username = argv[2];
    char *password = argv[3];
    char *amount_s = argv[4];

    int grams = atoi(amount_s);

    return transaction(dealer_name, username, password, grams);

}