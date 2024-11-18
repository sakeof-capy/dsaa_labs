#ifndef ARRAYBST_HPP
#define ARRAYBST_HPP
#include <cstddef>
#include <optional>
#include <stdexcept>
#include <utility>
#include <vector>



struct NodeId
{
public:
    std::size_t node_ndx;
};

template<typename Key, typename Value>
struct Node
{
    Key key;
    Value val;
    NodeId id{};
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

public:
    void insert(Key&& key, Value&& value)
    {
        insert_and_get_id(std::move(key), std::move(value));
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
            const NodeId id_to_insert = NodeId { .node_ndx = 0 };
            this->root_id_ = id_to_insert;

            this->array_.push_back(node_t {
                .key = std::move(key),
                .val = std::move(value),
                .id = id_to_insert,
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
                .id = id_to_insert,
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

    Node<Key, Value>& get_node_by_id(NodeId node_id)
    {
        return this->array_.at(node_id.node_ndx);
    }

    const Node<Key, Value>& get_root_node() const
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

private:
    std::vector<Node<Key, Value>> array_;
    std::optional<NodeId> root_id_;
};

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
