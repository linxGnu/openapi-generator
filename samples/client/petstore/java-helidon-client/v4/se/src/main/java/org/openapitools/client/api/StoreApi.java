/*
 * OpenAPI Petstore
 * This spec is mainly for testing Petstore server and contains fake endpoints, models. Please do not use this for any other purpose. Special characters: \" \\
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

package org.openapitools.client.api;

import org.openapitools.client.ApiResponse;
import java.util.List;
import java.util.Map;
import org.openapitools.client.model.Order;

/**
 * OpenAPI Petstore
 *
 * <p>This spec is mainly for testing Petstore server and contains fake endpoints, models. Please do not use this for any other purpose. Special characters: \" \\
 */
public interface StoreApi {

 /**
  * Delete purchase order by ID
  * For valid response try integer IDs with value &lt; 1000. Anything above 1000 or nonintegers will generate API errors
  * @param orderId ID of the order that needs to be deleted (required)
  * @return {@code ApiResponse<Void>}
  */
  ApiResponse<Void> deleteOrder(String orderId);

 /**
  * Returns pet inventories by status
  * Returns a map of status codes to quantities
  * @return {@code ApiResponse<Map<String, Integer>>}
  */
  ApiResponse<Map<String, Integer>> getInventory();

 /**
  * Find purchase order by ID
  * For valid response try integer IDs with value &lt;&#x3D; 5 or &gt; 10. Other values will generate exceptions
  * @param orderId ID of pet that needs to be fetched (required)
  * @return {@code ApiResponse<Order>}
  */
  ApiResponse<Order> getOrderById(Long orderId);

 /**
  * Place an order for a pet
  * 
  * @param order order placed for purchasing the pet (required)
  * @return {@code ApiResponse<Order>}
  */
  ApiResponse<Order> placeOrder(Order order);

}
