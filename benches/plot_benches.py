import numpy as np
import pandas
import matplotlib.pyplot as plt
import seaborn as sns


MARKERS = ['o', '*', '^']


def analysis(tree_size, tree_types, folder, overall_fig_name, compare_fig_name):
    marker = MARKERS[:len(tree_types)]

    mean_values = []
    fig, all_ax = plt.subplots(3, 2, figsize=(12, 16))
    for i, size in enumerate(tree_size):
        data_to_plot = []
        for type in tree_types:
            data = pandas.read_csv("../target/criterion/{}/{}/{}/base/raw.csv".format(folder, type, i))
            times = np.array(data['sample_measured_value'].to_list())
            # delta_times = times[1:] - times[:-1]
            counts = np.array(data['iteration_count'].to_list())
            # delta_counts = counts[1:] - counts[:-1]
            values = times / counts
            data_to_plot.append(values)
        mean_values.append([d.mean() for d in data_to_plot])

        # fig, ax = plt.subplots(figsize=(10, 8))
        ax = all_ax.item(i)
        ax.violinplot(data_to_plot, showextrema=False, showmedians=True)
        ax.set_xticks(np.arange(len(tree_types)).astype(np.int) + 1)
        ax.set_xticklabels(tree_types)
        ax.set_ylabel("time({})".format(data['unit'][0]), fontsize=10)
        ax.xaxis.set_tick_params(labelsize=10)
        ax.yaxis.set_tick_params(labelsize=6)
        ax.set_title("Tree Size = {}".format(size), fontsize=12)
        # plt.show()
        # plt.savefig("../target/criterion/{}.png".format(size))
    all_ax[-1, -1].axis('off')
    # plt.savefig("../target/criterion/compare.png")
    plt.savefig("../target/criterion/{}.png".format(compare_fig_name))
    # plt.show()
    fig, ax = plt.subplots(figsize=(10, 8))
    mean_values = np.array(mean_values)
    for i, (type, m) in enumerate(zip(tree_types, marker)):
        ax.plot(mean_values[:, i], label=type, marker=m)
    ax.set_xlabel("Tree Size", fontsize=20)
    ax.set_ylabel("Time(ns)", fontsize=20)
    plt.xticks(np.arange(len(tree_size)).astype(np.int), tree_size)
    ax.xaxis.set_tick_params(labelsize=13)
    ax.yaxis.set_tick_params(labelsize=13)
    plt.legend(loc="upper left", prop={'size': 20})
    # plt.show()
    plt.savefig("../target/criterion/{}.png".format(overall_fig_name))


def main():
    tree_size = [10000, 40000, 70000, 100000, 130000]
    tree_types = ["BST", "AVL", "RBT"]
    analysis(tree_size, tree_types, "Compare_10Sample", "overall_10_sample", "compare_10_sample")
    tree_types = ["AVL", "RBT"]
    analysis(tree_size, tree_types, "Compare", "overall", "compare")
    tree_types = ["BST", "AVL", "RBT"]
    analysis(tree_size, tree_types, "Compare_insert_delete", "overall_insert_delete", "compare_insert_delete")


if __name__ == '__main__':
    main()
