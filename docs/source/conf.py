# -*- coding: utf-8 -*-
# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html
import os
import sys

sys.path.insert(0, os.path.abspath("../src/py"))

from types import FunctionType, MethodType, ModuleType
from typing import Any, Dict, Literal, Type, Union

import sphinx.application

import rust_geodistances  # for autodoc
from rust_geodistances import config

config.env.SPHINX_IS_BUILDING = 1

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "Geodistance Calculations"
copyright = "2022, Denny Wong"
author = "Denny Wong"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

# napoleon for Numpy Docs
extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.napoleon",
]

templates_path = ["_templates"]
exclude_patterns = [
    "forestreet_cache/**",
]

autodoc_member_order = "bysource"

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "pydata_sphinx_theme"
html_static_path = ["_static"]
html_css_files = ["sig_word-break.css"]
# html_logo = "https://forestreet.com/wp-content/themes/Forestreet/assets/img/logo.svg"
html_theme_options = {
    "icon_links": [
        {
            "name": "GitHub Repository",
            "url": "https://github.com/denwong47/rust_primes",
            "icon": "fab fa-github-square",
        },
        # {
        #     "name": "GitLab",
        #     "url": "https://gitlab.com/<your-org>/<your-repo>",
        #     "icon": "fab fa-gitlab",
        # },
        # {
        #     "name": "Twitter",
        #     "url": "https://twitter.com/<your-handle>",
        #     "icon": "fab fa-twitter-square",
        # },
    ],
    "favicons": [
        # {
        #     "rel": "icon",
        #     "sizes": "16x16",
        #     "href": "https://forestreet.com/wp-content/themes/Forestreet/assets/img/favicon.ico",
        # },
        #   {
        #      "rel": "icon",
        #      "sizes": "32x32",
        #      "href": "favicon-32x32.png",
        #   },
        #   {
        #      "rel": "apple-touch-icon",
        #      "sizes": "180x180",
        #      "href": "apple-touch-icon-180x180.png"
        #   },
    ],
}

# -- Options for autodoc -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/extensions/autodoc.html

SKIP_MEMBERS = (rust_geodistances.lib_rust_geodistances,)

# def should_skip_member(
#     app: sphinx.application.Sphinx,
#     what: Literal["module", "class", "exception", "function", "method", "attribute"],
#     name: str,
#     obj: Union[
#         ModuleType,
#         Type,
#         BaseException,
#         FunctionType,
#         MethodType,
#         Any, # Attribute; we can't avoid using Any here.
#     ],
#     skip: bool,
#     options: Dict[
#         Literal["inherited_members", "undoc_members", "show_inheritance", "noindex"],
#         Any
#     ],
# ):
#     """
#     Implements a skip member check.

#     Refer to
#     `Sphinx Reference<https://www.sphinx-doc.org/en/master/usage/extensions/autodoc.html#event-autodoc-skip-member>`_
#     to implementation details.
#     """
#     if obj in SKIP_MEMBERS:
#         print(f"\033[1mmanually skipping member: \033[22m{repr(obj)}")
#     return obj in SKIP_MEMBERS


# def setup(app):
#     """
#     This function will be called by autodoc if exists.
#     """
#     app.connect('autodoc-skip-member', should_skip_member)
