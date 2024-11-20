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
bool operator==(const NodeId<Key, Value>& a, const NodeId<Key, Value>& b)
{
    return a.node.lock().get() == b.node.lock().get();
}

template<typename Key, typename Value>
class LinkedBST
{
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

    const Value& get_by_id(const NodeId node_id) const
    {
        return this->get_node_by_id(node_id, "get_by_id")->val;
    }

    Value& get_by_id(const NodeId node_id)
    {
        return this->get_node_by_id(node_id, "get_by_id")->val;
    }

    std::optional<NodeId> get_left_son_id(const NodeId node_id) const
    {
        try
        {
            return NodeId { .node = this->get_node_by_id(node_id, "get_left_son_id")->left_son_id };
        }
        catch (...)
        {
            return std::nullopt;
        }
    }

    std::optional<NodeId> get_right_son_id(const NodeId node_id) const
    {
        try
        {
            return NodeId { .node = this->get_node_by_id(node_id, "get_right_son_id")->right_son_id };
        }
        catch (...)
        {
            return std::nullopt;
        }
    }

    Value& get_left_son(const NodeId node_id)
    {
        return this->get_node_by_id(*this->get_left_son_id(node_id), "get_left_son")->val;
    }

    Value& get_right_son(const NodeId node_id)
    {
        return this->get_node_by_id(*this->get_right_son_id(node_id), "get_right_son")->val;
    }

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
        return get_node_by_id(*get_root_id(), "get_root")->val;
    }

    std::optional<NodeId> get_parent_id(const NodeId node_id) const
    {
        try
        {
            return NodeId { .node = this->get_node_by_id(node_id, "get_parent_id")->parent_id };
        }
        catch (...)
        {
            return std::nullopt;
        }
    }

    Value& get_parent(const NodeId node_id)
    {
        return this->get_node_by_id(*this->get_parent_id(node_id), "get_parent")->val;
    }

    const Value& get_parent(const NodeId node_id) const
    {
        return this->get_node_by_id(*this->get_parent_id(node_id), "get_parent")->val;
    }

    std::optional<NodeId> get_left_uncle_id(const NodeId node_id) const
    {
        try
        {
            const NodeId parent_id = *this->get_parent_id(node_id);
            const NodeId grand_parent_id = *this->get_parent_id(parent_id);
            return this->get_left_son_id(grand_parent_id);
        }
        catch (...)
        {
            return std::nullopt;
        }
    }

    std::optional<NodeId> get_right_uncle_id(const NodeId node_id) const
    {
        try
        {
            const NodeId parent_id = *this->get_parent_id(node_id);
            const NodeId grand_parent_id = *this->get_parent_id(parent_id);
            return this->get_right_son_id(grand_parent_id);
        }
        catch (...)
        {
            return std::nullopt;
        }
    }

    Value& get_left_uncle(const NodeId node_id)
    {
        return this->get_node_by_id(*this->get_left_uncle_id(node_id), "get_left_uncle")->val;
    }

    Value& get_right_uncle(const NodeId node_id)
    {
        return this->get_node_by_id(*this->get_right_uncle_id(node_id), "get_right_uncle")->val;
    }

    bool is_left_son(NodeId node_id) const
    {
        const NodeId parent_id = *this->get_parent_id(node_id);
        const NodeId left_son_id = *this->get_left_son_id(parent_id);
        return left_son_id == node_id;
    }

    bool is_right_son(const NodeId node_id) const
    {
        const NodeId parent_id = *this->get_parent_id(node_id);
        const NodeId right_son_id = *this->get_right_son_id(parent_id);
        return right_son_id == node_id;
    }

    void left_rotate(const NodeId node_id)
    {
        const NodeId son_id = this->get_right_son_id(node_id).value();

        node_ptr_t node = this->get_node_by_id(node_id, "left_rotate");
        node_ptr_t son = this->get_node_by_id(son_id, "left_rotate");

        node->right_son_id = son->left_son_id;
        if (son->left_son_id) {
            son->left_son_id->parent_id = node;
        }

        son->parent_id = node->parent_id;

        if (node->parent_id.expired()) {
            this->root_ = son;
        } else {
            node_ptr_t parent = this->get_node_by_id(NodeId{node->parent_id}, "left_rotate");
            if (parent->left_son_id == node) {
                parent->left_son_id = son;
            } else {
                parent->right_son_id = son;
            }
        }

        son->left_son_id = node;
        node->parent_id = son;
    }

    void right_rotate(const NodeId node_id)
    {
        const NodeId son_id = this->get_left_son_id(node_id).value();

        node_ptr_t node = this->get_node_by_id(node_id, "right_rotate");
        node_ptr_t son = this->get_node_by_id(son_id, "right_rotate");

        node->left_son_id = son->right_son_id;
        if (son->right_son_id) {
            son->right_son_id->parent_id = node;
        }

        son->parent_id = node->parent_id;

        if (node->parent_id.expired()) {
            this->root_ = son;
        } else {
            node_ptr_t parent = this->get_node_by_id(NodeId{node->parent_id}, "right_rotate");
            if (parent->left_son_id == node) {
                parent->left_son_id = son;
            } else {
                parent->right_son_id = son;
            }
        }

        son->right_son_id = node;
        node->parent_id = son;
    }

private:
    node_ptr_t get_node_by_id(NodeId node_id, const std::string& function_name) const
    {
        node_ptr_t shared_node = node_id.node.lock();

        if (!shared_node)
        {
            return this->root_;
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

#include <iomanip> // For std::setw

// Helper function to stringify a subtree for LinkedBST
template<typename Key, typename Value>
std::string stringify_subtree(
    const typename LinkedBST<Key, Value>::NodeId& node_id,
    const std::string& prefix,
    bool is_left,
    size_t depth)
{
    // Base case: Prevent excessive recursion or handle null nodes
    if (depth > 100 || node_id.node.expired()) {
        return prefix + (is_left ? "|--" : "|__") + " NIL\n";
    }

    // Access the current node
    auto node = node_id.node.lock();
    std::ostringstream result;

    // Current node representation
    result << prefix
           << (is_left ? "|--" : "|__")
           << " " << node->key << " -> " << node->val << "\n";

    // Update the prefix for child nodes
    std::string child_prefix = prefix + (is_left ? "|   " : "    ");

    // Process left child
    if (node->left_son_id) {
        result << stringify_subtree<Key, Value>(
            typename LinkedBST<Key, Value>::NodeId{ .node = node->left_son_id },
            child_prefix,
            true,
            depth + 1
        );
    } else {
        result << child_prefix << "|-- NIL\n";
    }

    // Process right child
    if (node->right_son_id) {
        result << stringify_subtree<Key, Value>(
            typename LinkedBST<Key, Value>::NodeId{ .node = node->right_son_id },
            child_prefix,
            false,
            depth + 1
        );
    } else {
        result << child_prefix << "|__ NIL\n";
    }

    return result.str();
}

// Overload << operator for LinkedBST
template<typename Key, typename Value>
std::ostream& operator<<(std::ostream& os, const LinkedBST<Key, Value>& bst) {
    if (bst.get_root_id().has_value()) {
        os << stringify_subtree<Key, Value>(
            bst.get_root_id().value(),
            "",
            false,
            0
        );
    } else {
        os << "NIL\n";
    }
    return os;
}


#endif // LISTBST_HPP
