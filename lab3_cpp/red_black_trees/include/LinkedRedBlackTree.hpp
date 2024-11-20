#ifndef LINKEDREDBLACKTREE_HPP
#define LINKEDREDBLACKTREE_HPP

#include <utility>
#include "ListBST.hpp"

enum class Color { RED, BLACK };

template<typename Key, typename Value>
class LinkedRedBlackTree {
private:
    struct RBNode {
        Value value;
        Color color;
    };

public:
    using NodeId = typename LinkedBST<Key, RBNode>::NodeId;

private:
    void left_rotate(NodeId node_id) {
        bst_.left_rotate(node_id);
    }

    void right_rotate(NodeId node_id) {
        bst_.right_rotate(node_id);
    }

    void fix_insertion(const typename LinkedBST<Key, RBNode>::NodeId& node_id)
    {
        auto current = node_id;

        while (auto node = current.node.lock()) {  // Check if node is still valid
            if (node->val.color == Color::RED) {
                auto parent_id = bst_.get_parent_id(current);
                if (!parent_id) break; // Parent is null
                auto grandparent_id = bst_.get_parent_id(parent_id.value());
                if (!grandparent_id) break; // Grandparent is null

                if (bst_.is_left_son(parent_id.value())) {
                    auto uncle_id = bst_.get_right_son_id(grandparent_id.value());
                    if (uncle_id && bst_.get_by_id(uncle_id.value()).color == Color::RED) {
                        bst_.get_by_id(parent_id.value()).color = Color::BLACK;
                        bst_.get_by_id(uncle_id.value()).color = Color::BLACK;
                        bst_.get_by_id(grandparent_id.value()).color = Color::RED;
                        current = grandparent_id.value();
                    } else {
                        if (bst_.is_right_son(current)) {
                            current = parent_id.value();
                            bst_.left_rotate(current);
                        }
                        bst_.get_by_id(parent_id.value()).color = Color::BLACK;
                        bst_.get_by_id(grandparent_id.value()).color = Color::RED;
                        bst_.right_rotate(grandparent_id.value());
                    }
                } else {
                    auto uncle_id = bst_.get_left_son_id(grandparent_id.value());
                    if (uncle_id && bst_.get_by_id(uncle_id.value()).color == Color::RED) {
                        bst_.get_by_id(parent_id.value()).color = Color::BLACK;
                        bst_.get_by_id(uncle_id.value()).color = Color::BLACK;
                        bst_.get_by_id(grandparent_id.value()).color = Color::RED;
                        current = grandparent_id.value();
                    } else {
                        if (bst_.is_left_son(current)) {
                            current = parent_id.value();
                            bst_.right_rotate(current);
                        }
                        bst_.get_by_id(parent_id.value()).color = Color::BLACK;
                        bst_.get_by_id(grandparent_id.value()).color = Color::RED;
                        bst_.left_rotate(grandparent_id.value());
                    }
                }
            }
            break;
        }

        // Ensure the root is black after insertion
        if (auto root_id = bst_.get_root_id(); root_id.has_value()) {
            bst_.get_by_id(root_id.value()).color = Color::BLACK;
        } else {
            std::cerr << "Root node is invalid!" << std::endl;
        }
    }


public:
    void insert(Key&& key, Value&& value) {
        const NodeId inserted_id = bst_.insert_and_get_id(
            std::move(key),
            RBNode { .value = std::move(value), .color = Color::RED }
        );
        fix_insertion(inserted_id);
    }

    const Value& get(const Key& key) const {
        return bst_.get(key).value;
    }

private:
    template<typename KKey, typename VValue>
    friend std::ostream& operator<<(std::ostream& os, const LinkedRedBlackTree<KKey, VValue>& bst);

    friend std::ostream& operator<<(std::ostream& os, const RBNode& node)
    {
        return os << '[' << node.value << " | " << node.color << ']';
    }

private:
    LinkedBST<Key, RBNode> bst_;
};


template<typename Key, typename Value>
std::ostream& operator<<(std::ostream& os, const LinkedRedBlackTree<Key, Value>& bst)
{
    return os << bst.bst_;
}

inline std::ostream& operator<<(std::ostream& os, const Color& clr)
{
    switch (clr)
    {
        case Color::RED :
            return os << 'R';
        case Color::BLACK :
            return os << 'B';
        default:
            std::unreachable();
    }

}

#endif //LINKEDREDBLACKTREE_HPP
