#ifndef REDBLACKTREE_HPP
#define REDBLACKTREE_HPP

#include "ArrayBST.hpp"

enum class Color { Red, Black };

template<typename Key, typename Value>
class RedBlackTree {
private:
    struct RBNode {
        Value value;
        Color color;
    };

    void left_rotate(NodeId node_id) {
        bst_.left_rotate(node_id);
    }

    void right_rotate(NodeId node_id) {
        bst_.right_rotate(node_id);
    }

    void fix_insertion(NodeId inserted_id) {
        auto get_parent_color = [this](NodeId id) -> Color {
            auto parent_id = bst_.get_parent_id(id);
            if (!parent_id.has_value())
                return Color::Black; // Null parent is considered black.
            return bst_.get_by_id(parent_id.value()).color;
        };

        while (inserted_id.node_ndx != bst_.get_root_id().value().node_ndx &&
               get_parent_color(inserted_id) == Color::Red) {
            NodeId parent_id = bst_.get_parent_id(inserted_id).value();
            NodeId grandparent_id = bst_.get_parent_id(parent_id).value();

            if (bst_.is_left_son(parent_id)) {
                NodeId uncle_id = bst_.get_right_son_id(grandparent_id).value_or(NodeId{static_cast<std::size_t>(-1)});
                if (uncle_id.node_ndx != static_cast<std::size_t>(-1) &&
                    bst_.get_by_id(uncle_id).color == Color::Red) {
                    bst_.get_by_id(parent_id).color = Color::Black;
                    bst_.get_by_id(uncle_id).color = Color::Black;
                    bst_.get_by_id(grandparent_id).color = Color::Red;
                    inserted_id = grandparent_id;
                } else {
                    if (bst_.is_right_son(inserted_id)) {
                        inserted_id = parent_id;
                        left_rotate(inserted_id);
                        parent_id = bst_.get_parent_id(inserted_id).value();
                        grandparent_id = bst_.get_parent_id(parent_id).value();
                    }
                    bst_.get_by_id(parent_id).color = Color::Black;
                    bst_.get_by_id(grandparent_id).color = Color::Red;
                    right_rotate(grandparent_id);
                }
            } else {
                NodeId uncle_id = bst_.get_left_son_id(grandparent_id).value_or(NodeId{static_cast<std::size_t>(-1)});
                if (uncle_id.node_ndx != static_cast<std::size_t>(-1) &&
                    bst_.get_by_id(uncle_id).color == Color::Red) {
                    bst_.get_by_id(parent_id).color = Color::Black;
                    bst_.get_by_id(uncle_id).color = Color::Black;
                    bst_.get_by_id(grandparent_id).color = Color::Red;
                    inserted_id = grandparent_id;
                } else {
                    if (bst_.is_left_son(inserted_id)) {
                        inserted_id = parent_id;
                        right_rotate(inserted_id);
                        parent_id = bst_.get_parent_id(inserted_id).value();
                        grandparent_id = bst_.get_parent_id(parent_id).value();
                    }
                    bst_.get_by_id(parent_id).color = Color::Black;
                    bst_.get_by_id(grandparent_id).color = Color::Red;
                    left_rotate(grandparent_id);
                }
            }
        }
        bst_.get_by_id(bst_.get_root_id().value()).color = Color::Black;
    }

public:
    void insert(Key&& key, Value&& value) {
        const NodeId inserted_id = bst_.insert_and_get_id(
            std::move(key),
            RBNode { .value = std::move(value), .color = Color::Red }
        );
        fix_insertion(inserted_id);
    }

    const Value& get(const Key& key) const {
        return bst_.get(key).value;
    }

private:
    template<typename KKey, typename VValue>
    friend std::ostream& operator<<(std::ostream& os, const RedBlackTree<KKey, VValue>& bst);

    friend std::ostream& operator<<(std::ostream& os, const RBNode& node)
    {
        return os << '[' << node.value << " | " << node.color << ']';
    }

private:
    ArrayBST<Key, RBNode> bst_;
};


template<typename Key, typename Value>
std::ostream& operator<<(std::ostream& os, const RedBlackTree<Key, Value>& bst)
{
    return os << bst.bst_;
}

std::ostream& operator<<(std::ostream& os, const Color& clr)
{
    switch (clr)
    {
        case Color::Red :
            return os << 'R';
        case Color::Black :
            return os << 'B';
        default:
            std::unreachable();
    }

}


#endif // REDBLACKTREE_HPP
