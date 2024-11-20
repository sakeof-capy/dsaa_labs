#ifndef ARRAYBST_HPP
#define ARRAYBST_HPP
#include <cstddef>
#include <optional>
#include <stdexcept>
#include <utility>
#include <vector>
#include <ostream>
#include <sstream>
#include <string>

struct NodeId
{
public:
    std::size_t node_ndx;
};

inline bool operator==(const NodeId lhs, const NodeId rhs)
{
    return lhs.node_ndx == rhs.node_ndx;
}

template<typename Key, typename Value>
struct Node
{
    Key key;
    Value val;
    std::optional<NodeId> parent_id;
    std::optional<NodeId> left_son_id;
    std::optional<NodeId> right_son_id;
};

enum class SonType
{
    Left,
    Right,
};

struct InsertionResult
{
    bool root{};
    std::optional<std::pair<NodeId, SonType>> son_of;
    std::optional<NodeId> node_already_exists;
};

template<typename Key, typename Value>
class ArrayBST
{
public:
    using node_t = Node<Key, Value>;
    using NodeId = NodeId;

public:
    void insert(Key&& key, Value&& value)
    {
        static_cast<void>(insert_and_get_id(std::move(key), std::move(value)));
    }

    const Value& get(const Key& key) const
    {
        const InsertionResult result = this->find_insertion_place(key);

        if (!result.node_already_exists.has_value())
        {
            throw std::invalid_argument("Key not found");
        }

        const NodeId node_id = result.node_already_exists.value();
        const node_t& node = this->get_node_by_id(node_id);

        return node.val;
    }

    NodeId insert_and_get_id(Key&& key, Value&& value)
    {
        const InsertionResult result = this->find_insertion_place(key);
        NodeId inserted_node_id{};

        if (result.root)
        {
            constexpr NodeId id_to_insert = NodeId { .node_ndx = 0 };
            this->root_id_ = id_to_insert;

            this->array_.push_back(node_t {
                .key = std::move(key),
                .val = std::move(value),
                .parent_id = std::nullopt,
                .left_son_id = std::nullopt,
                .right_son_id = std::nullopt,
            });

            inserted_node_id = id_to_insert;
        }

        if (result.son_of.has_value())
        {
            const auto [parent_id, son_type] = result.son_of.value();
            const NodeId id_to_insert { .node_ndx = this->array_.size() };

            this->array_.push_back(node_t {
                .key = std::move(key),
                .val = std::move(value),
                .parent_id = parent_id,
                .left_son_id = std::nullopt,
                .right_son_id = std::nullopt,
            });

            node_t& parent = this->get_node_by_id(parent_id);

            switch (son_type)
            {
                case SonType::Left:
                    parent.left_son_id = id_to_insert;
                    break;
                case SonType::Right:
                    parent.right_son_id = id_to_insert;
                    break;
                default:
                    break;
            }

            inserted_node_id = id_to_insert;
        }

        if (result.node_already_exists.has_value())
        {
            const NodeId existent_id = result.node_already_exists.value();
            node_t& existent_node = this->get_node_by_id(existent_id);
            existent_node.val = std::move(value);
            inserted_node_id = existent_id;
        }

        return inserted_node_id;
    }

    const Value& get_by_id(const NodeId node_id) const
    {
        return this->get_node_by_id(node_id).val;
    }

    Value& get_by_id(const NodeId node_id)
    {
        return this->get_node_by_id(node_id).val;
    }

    std::optional<NodeId> get_left_son_id(const NodeId node_id) const
    {
        try
        {
            return this->get_node_by_id(node_id).left_son_id;
        }
        catch (const std::out_of_range&)
        {
            return std::nullopt;
        }
    }

    std::optional<NodeId> get_right_son_id(const NodeId node_id) const
    {
        try
        {
            return this->get_node_by_id(node_id).right_son_id;
        }
        catch (const std::out_of_range&)
        {
            return std::nullopt;
        }
    }

    Value& get_left_son(const NodeId node_id)
    {
        std::optional<NodeId> left_son_id_opt = this->get_left_son_id(node_id);
        return this->extract_node_by_id_opt(std::move(left_son_id_opt), "Left son not found");
    }

    Value& get_right_son(const NodeId node_id)
    {
        std::optional<NodeId> right_son_id_opt = this->get_right_son_id(node_id);
        return this->extract_node_by_id_opt(std::move(right_son_id_opt), "Right son not found");
    }

    std::optional<NodeId> get_root_id() const
    {
        return this->root_id_;
    }

    Value& get_root()
    {
        std::optional<NodeId> root_id = this->get_root_id();
        return extract_node_by_id_opt(std::move(root_id), "Root not found");
    }

    std::optional<NodeId> get_parent_id(const NodeId node_id) const
    {
        try
        {
            return this->get_node_by_id(node_id).parent_id;
        }
        catch (const std::out_of_range&)
        {
            return std::nullopt;
        }
    }

    Value& get_parent(const NodeId node_id)
    {
        std::optional<NodeId> parent_id_opt = this->get_parent_id(node_id);
        return this->extract_node_by_id_opt(std::move(parent_id_opt), "Parent not found");
    }

    const Value& get_parent(const NodeId node_id) const
    {
        std::optional<NodeId> parent_id_opt = this->get_parent_id(node_id);
        return this->extract_node_by_id_opt(std::move(parent_id_opt), "Parent not found");
    }

    std::optional<NodeId> get_left_uncle_id(NodeId node_id) const
    {
        return this->get_parent_id(node_id)
            .and_then([this](const NodeId parent_id) {
                return this->get_parent_id(parent_id);
            })
            .and_then([this](const NodeId grand_parent_id) {
                return this->get_left_son_id(grand_parent_id);
            });
    }

    std::optional<NodeId> get_right_uncle_id(NodeId node_id) const
    {
        return this->get_parent_id(node_id)
            .and_then([this](const NodeId parent_id) {
                return this->get_parent_id(parent_id);
            })
            .and_then([this](const NodeId grand_parent_id) {
                return this->get_right_son_id(grand_parent_id);
            });
    }

    Value& get_left_uncle(NodeId node_id)
    {
        std::optional<NodeId> left_uncle_id_opt = get_left_uncle_id(node_id);
        return extract_node_by_id_opt(std::move(left_uncle_id_opt), "Left uncle not found");
    }

    Value& get_right_uncle(NodeId node_id)
    {
        std::optional<NodeId> right_uncle_id_opt = get_right_uncle_id(node_id);
        return extract_node_by_id_opt(std::move(right_uncle_id_opt), "Right uncle not found");
    }

    bool is_left_son(const NodeId node_id) const
    {
        return this->get_parent_id(node_id)
            .and_then([this](const NodeId parent_id) {
                return this->get_left_son_id(parent_id);
            })
            .and_then([this, node_id](const NodeId left_son_id) {
                return std::optional(left_son_id == node_id);
            })
            .value_or(false);
    }

    bool is_right_son(NodeId node_id) const
    {
        return this->get_parent_id(node_id)
            .and_then([this](const NodeId parent_id) {
                return this->get_right_son_id(parent_id);
            })
            .and_then([this, node_id](const NodeId right_son_id) {
                return std::optional(right_son_id == node_id);
            })
            .value_or(false);
    }

    void left_rotate(NodeId node_id)
    {
        const NodeId son_id = this->get_right_son_id(node_id).value();
        const std::optional<NodeId> sons_left_son_id = this->get_left_son_id(son_id);
        const bool is_left_son = this->is_left_son(node_id);

        node_t& node = this->get_node_by_id(node_id);
        const std::optional<NodeId> node_parent_id_opt = node.parent_id;

        node.parent_id = son_id;
        node.right_son_id = sons_left_son_id;

        try
        {
            node_t& son = this->get_node_by_id(son_id);
            son.parent_id = node_parent_id_opt;
            son.left_son_id = node_id;
        }
        catch(...) {}

        if (node_parent_id_opt.has_value())
        {
            const NodeId node_parent_id = node_parent_id_opt.value();

            try
            {
                node_t& parent = this->get_node_by_id(node_parent_id);

                if (is_left_son)
                {
                    parent.left_son_id = son_id;
                }
                else
                {
                    parent.right_son_id = son_id;
                }
            }
            catch(...) {}

            return;
        }

        this->root_id_ = son_id;
    }

    void right_rotate(NodeId node_id)
    {
        const NodeId son_id = this->get_left_son_id(node_id).value();
        const std::optional<NodeId> sons_right_son_id = this->get_right_son_id(son_id);
        const bool is_left_son = this->is_left_son(node_id);

        node_t& node = this->get_node_by_id(node_id);
        const std::optional<NodeId> node_parent_id_opt = node.parent_id;

        node.parent_id = son_id;
        node.left_son_id = sons_right_son_id;

        try
        {
            node_t& son = this->get_node_by_id(son_id);
            son.parent_id = node_parent_id_opt;
            son.right_son_id = node_id;
        }
        catch(...) {}

        if (node_parent_id_opt.has_value())
        {
            const NodeId node_parent_id = node_parent_id_opt.value();

            try
            {
                node_t& parent = this->get_node_by_id(node_parent_id);

                if (is_left_son)
                {
                    parent.left_son_id = son_id;
                }
                else
                {
                    parent.right_son_id = son_id;
                }
            }
            catch(...) {}

            return;
        }

        this->root_id_ = son_id;
    }

private:
    InsertionResult find_insertion_place(const Key& key) const
    {
        const std::optional<NodeId> root_id = this->root_id_;
        NodeId current_id = root_id.value_or(NodeId { .node_ndx = static_cast<std::size_t>(-1) });

        while (this->node_with_id_exists(current_id))
        {
            const node_t& current_node = this->get_node_by_id(current_id);

            if (key < current_node.key)
            {
                if (current_node.left_son_id.has_value())
                {
                    current_id = current_node.left_son_id.value();
                }
                else
                {
                    return InsertionResult {
                        .son_of = std::pair { current_id, SonType::Left },
                    };
                }
            }
            else if (key > current_node.key)
            {
                if (current_node.right_son_id.has_value())
                {
                    current_id = current_node.right_son_id.value();
                }
                else
                {
                    return InsertionResult {
                        .son_of = std::pair { current_id, SonType::Right },
                    };
                }
            }
            else
            {
                return InsertionResult {
                    .node_already_exists = current_id,
                };
            }
        }

        return InsertionResult {
            .root = true
        };
    }

    const Node<Key, Value>& get_node_by_id(NodeId node_id) const
    {
        return this->array_.at(node_id.node_ndx);
    }

    node_t& get_node_by_id(NodeId node_id)
    {
        return this->array_.at(node_id.node_ndx);
    }

    const node_t& get_root_node() const
    {
        if (!this->root_id_.has_value())
        {
            throw std::invalid_argument("Invalid node id");
        }

        return this->get_node_by_id(this->root_id_.value());
    }

    bool node_with_id_exists(NodeId node_id) const
    {
        return node_id.node_ndx < this->array_.size();
    }

    Value& extract_node_by_id_opt(std::optional<NodeId>&& node_id_opt, const char* err_msg)
    {
        if (!node_id_opt.has_value())
        {
            throw std::invalid_argument(err_msg);
        }

        return this->get_by_id(node_id_opt.value());
    }

    const Value& extract_node_by_id_opt(std::optional<NodeId>&& node_id_opt, const char* err_msg) const
    {
        if (!node_id_opt.has_value())
        {
            throw std::invalid_argument(err_msg);
        }

        return this->get_by_id(node_id_opt.value());
    }

private:
    template<typename KKey, typename VValue>
    friend std::ostream& operator<<(std::ostream& os, const ArrayBST<KKey, VValue>& bst);

private:
    std::vector<Node<Key, Value>> array_;
    std::optional<NodeId> root_id_;
};

template<typename Key, typename Value>
std::string stringify_subtree(
    const NodeId& node_id,
    const std::vector<Node<Key, Value>>& array,
    const std::string& prefix,
    bool is_left,
    size_t depth)
{
    // Base case: Prevent excessive recursion or out-of-bounds access
    if (depth > 100 || node_id.node_ndx >= array.size()) {
        return prefix + (is_left ? "|--" : "|__") + " NIL\n";
    }

    const auto& node = array[node_id.node_ndx];
    std::ostringstream result;

    // Current node representation
    result << prefix
           << (is_left ? "|--" : "|__")
           << " " << node.key << " -> " << node.val << "\n";

    // Update the prefix for child nodes
    std::string child_prefix = prefix + (is_left ? "|   " : "    ");

    // Process left child
    if (node.left_son_id.has_value()) {
        result << stringify_subtree(node.left_son_id.value(), array, child_prefix, true, depth + 1);
    } else {
        result << child_prefix << "|-- NIL\n";
    }

    // Process right child
    if (node.right_son_id.has_value()) {
        result << stringify_subtree(node.right_son_id.value(), array, child_prefix, false, depth + 1);
    } else {
        result << child_prefix << "|__ NIL\n";
    }

    return result.str();
}

template<typename Key, typename Value>
std::ostream& operator<<(std::ostream& os, const ArrayBST<Key, Value>& bst) {
    if (bst.get_root_id().has_value()) {
        os << stringify_subtree(
            bst.get_root_id().value(),
            bst.array_,
            "",
            false,
            0
        );
    } else {
        os << "NIL\n";
    }
    return os;
}

/*
    void insert(Key&& key, Value&& value);
    const Value& get(const Key& key) const;

    NodeId insert_and_get_id(Key&& key, Value&& value);
    const Value& get_by_id(NodeId node_id) const;
    Value& get_by_id(NodeId node_id);
    std::optional<NodeId> get_left_son_id(NodeId node_id) const;
    std::optional<NodeId> get_right_son_id(NodeId node_id) const;
    Value& get_left_son(NodeId node_id);
    Value& get_right_son(NodeId node_id);
    std::optional<NodeId> get_root_id() const;
    Value& get_root();

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
 */

#endif //ARRAYBST_HPP
