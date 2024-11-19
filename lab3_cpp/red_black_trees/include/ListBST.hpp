#ifndef LISTBST_HPP
#define LISTBST_HPP
#include <memory>
#include <optional>
#include <stdexcept>
#include <sstream>
#include <string>
#include <iostream>

template<typename Key, typename Value>
struct Node {
    Key key;
    Value val;
    std::weak_ptr<Node> parent_id;
    std::shared_ptr<Node> left_son_id;
    std::shared_ptr<Node> right_son_id;
};

template<typename Key, typename Value>
struct NodeId {
    std::weak_ptr<Node<Key, Value>> node;
};

template<typename Key, typename Value>
class LinkedBST {
public:
    using node_t = Node<Key, Value>;
    using node_ptr_t = std::shared_ptr<node_t>;
    using NodeId = NodeId<Key, Value>;

public:
    void insert(Key&& key, Value&& value)
    {
        insert_and_get_id(std::move(key), std::move(value));
    }

    const Value& get(const Key& key) const
    {
        auto node = find_node(key);
        if (!node) {
            throw std::runtime_error("Key not found");
        }
        return node->val;
    }

    NodeId insert_and_get_id(Key&& key, Value&& value) {
        node_ptr_t new_node = std::make_shared<node_t>(node_t {
            std::move(key), std::move(value)
        });
        if (!root_) {
            root_ = new_node;
        } else {
            auto current = root_;
            std::shared_ptr<node_t> parent;
            while (current)
            {
                parent = current;
                if (new_node->key < current->key)
                {
                    current = current->left_son_id;
                }
                else if (new_node->key > current->key)
                {
                    current = current->right_son_id;
                }
                else
                {
                    throw std::runtime_error("Duplicate key");
                }
            }
            new_node->parent_id = parent;
            if (new_node->key < parent->key)
            {
                parent->left_son_id = new_node;
            }
            else
            {
                parent->right_son_id = new_node;
            }
        }
        return NodeId { new_node };
    }

    const Value& get_by_id(NodeId node_id) const;
    Value& get_by_id(NodeId node_id);
    std::optional<NodeId> get_left_son_id(NodeId node_id) const;
    std::optional<NodeId> get_right_son_id(NodeId node_id) const;
    Value& get_left_son(NodeId node_id);
    Value& get_right_son(NodeId node_id);

    std::optional<NodeId> get_root_id() const
    {
        if (!root_)
        {
            return std::nullopt;
        }

        return NodeId { .node = root_ };
    }

    Value& get_root()
    {
        return get_node_by_id(*get_root_id())->val;
    }

    std::optional<NodeId> get_parent_id(NodeId node_id) const;
    Value& get_parent(NodeId node_id);
    const Value& get_parent(NodeId node_id) const;
    std::optional<NodeId> get_left_uncle_id(NodeId node_id) const;
    std::optional<NodeId> get_right_uncle_id(NodeId node_id) const;
    Value& get_left_uncle(NodeId node_id);
    Value& get_right_uncle(NodeId node_id);
    bool is_left_son(NodeId node_id) const;
    bool is_right_son(NodeId node_id) const;

    void left_rotate(NodeId node_id);
    void right_rotate(NodeId node_id);

private:
    node_ptr_t get_node_by_id(NodeId node_id) const
    {
        node_ptr_t shared_node = node_id.node.lock();

        if (!shared_node)
        {
            throw std::runtime_error("Invalid NodeId");
        }

        return shared_node;
    }

    node_ptr_t find_node(const Key& key) const
    {
        auto current = root_;
        while (current)
        {
            if (key < current->key)
            {
                current = current->left_son_id;
            }
            else if (key > current->key)
            {
                current = current->right_son_id;
            }
            else
            {
                return current;
            }
        }
        return nullptr;
    }

private:
    template<typename KKey, typename VValue>
    friend std::ostream& operator<<(std::ostream& os, const LinkedBST<KKey, VValue>& bst);

private:
    node_ptr_t root_;
};


#endif // LISTBST_HPP
