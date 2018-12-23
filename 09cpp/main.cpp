#include <algorithm>
#include <list>
#include <iostream>
#include <vector>

using namespace std;

// TODO: just check the distance from begin/end instead of writing functions
namespace {
        template<typename T>
        void wrapping_advance(T& owner, typename T::iterator& it) {
                ++it;
                if (it == owner.end()) {
                        it = owner.begin();
                }
        }

        template<typename T>
        void wrapping_rewind(T& owner, typename T::iterator& it) {
                if (it == owner.begin()) {
                        it = owner.end();
                }
                --it;
        }
}

int main(int argc, char** argv) {
        const int n_players = 426;
        const int last_marble_value = 7205800;
        list<int> marbles;
        marbles.push_back(0);
        vector<long> scores(n_players);
        auto it = marbles.begin();
        for (int m_no = 1; m_no <= last_marble_value; ++m_no) {
                const int player_ix = m_no % n_players;
                if (m_no % 23 == 0) {
                        scores[player_ix] += m_no;
                        for (int i = 0; i < 7; ++i) {
                                wrapping_rewind(marbles, it);
                        }
                        auto to_erase(it);
                        it++;
                        scores[player_ix] += *to_erase;
                        marbles.erase(to_erase);
                } else {
                        wrapping_advance(marbles, it);
                        wrapping_advance(marbles, it);
                        it = marbles.insert(it, m_no);
                }
        }
        const long max_score = *max_element(scores.begin(), scores.end());
        cout << "Answer: " << max_score << '\n';
        return 0;
}
